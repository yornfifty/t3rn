#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{sp_runtime::DispatchError, traits::Get};
use frame_system::pallet_prelude::OriginFor;
pub use pallet::*;

use t3rn_abi::recode::{recode_bytes_with_descriptor, Codec};

#[cfg(test)]
mod tests;

use sp_std::vec::Vec;
use t3rn_abi::types::Bytes;
use t3rn_primitives::{
    light_client::LightClient, portal::Portal, xdns::Xdns, ChainId, GatewayVendor,
};

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use core::convert::TryInto;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::{vec, vec::Vec};
    use t3rn_primitives::{
        gateway::GatewayABIConfig, xdns::Xdns, ChainId, GatewayGenesisConfig, GatewayType,
        GatewayVendor, TokenSysProps,
    };
    use t3rn_types::sfx::Sfx4bId;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type LightClients: Get<Vec<(GatewayVendor, Box<dyn LightClient<Self>>)>>;

        type Xdns: Xdns<Self>;
        /// Type representing the weight of this pallet
        type WeightInfo: crate::weights::WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// Gateway was registered successsfully. [ChainId]
        GatewayRegistered(ChainId),
        /// Gateway owner was set successfully. [ChainId, Vec<u8>]
        SetOwner(ChainId, Vec<u8>),
        /// Gateway was set operational. [ChainId, bool]
        SetOperational(ChainId, bool),
        /// Header was successfully added
        HeaderSubmitted(GatewayVendor, Vec<u8>),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The creation of the XDNS record was not successful
        XdnsRecordCreationFailed,
        ///Specified Vendor is not implemented
        UnimplementedGatewayVendor,
        /// Gateway registration failed
        RegistrationError,
        /// The gateways vendor is not available, which is a result of a missing XDNS record.
        GatewayVendorNotFound,
        /// Finality Verifier owner can't be set.
        SetOwnerError,
        /// Finality Verifiers operational status can't be updated
        SetOperationalError,
        /// The header could not be added
        SubmitHeaderError,
        /// No gateway height could be found
        NoGatewayHeightAvailable,
        /// SideEffect confirmation failed
        SideEffectConfirmationFailed,
        /// Recoding failed
        SFXRecodeError,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn submit_headers(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            encoded_header_data: Vec<u8>,
        ) -> DispatchResult {
            let _ = ensure_signed(origin.clone())?;
            match_light_client_by_gateway_id::<T>(gateway_id)?
                .submit_headers(origin, encoded_header_data)?;
            Ok(())
        }
    }
}

// ToDo: this should come from XDNS
pub fn match_vendor_with_codec(vendor: GatewayVendor) -> Codec {
    match vendor {
        GatewayVendor::Rococo => Codec::Scale,
        GatewayVendor::Kusama => Codec::Scale,
        GatewayVendor::Polkadot => Codec::Scale,
        GatewayVendor::Ethereum => Codec::Rlp,
    }
}

pub fn match_light_client_by_gateway_id<T: Config>(
    gateway_id: ChainId,
) -> Result<Box<dyn LightClient<T>>, Error<T>> {
    let vendor = <T as Config>::Xdns::get_verification_vendor(&gateway_id)
        .map_err(|_| Error::<T>::GatewayVendorNotFound)?;
    match_light_client_by_vendor(vendor)
}

pub fn match_light_client_by_vendor<T: Config>(
    vendor: GatewayVendor,
) -> Result<Box<dyn LightClient<T>>, Error<T>> {
    let light_clients = <T as Config>::LightClients::get();
    let light_client = light_clients
        .into_iter()
        .find(|(v, _)| *v == vendor)
        .map(|(_, lc)| lc)
        .ok_or(Error::<T>::UnimplementedGatewayVendor)?;
    Ok(light_client)
}

impl<T: Config> Portal<T> for Pallet<T> {
    fn get_latest_finalized_header(gateway_id: ChainId) -> Result<Option<Bytes>, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.get_latest_finalized_header()
    }

    fn get_latest_finalized_height(
        gateway_id: ChainId,
    ) -> Result<Option<T::BlockNumber>, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.get_latest_finalized_height()
    }

    fn get_latest_updated_height(
        gateway_id: ChainId,
    ) -> Result<Option<T::BlockNumber>, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.get_latest_updated_height()
    }

    fn get_current_epoch(gateway_id: ChainId) -> Result<Option<u32>, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.get_current_epoch()
    }

    fn read_fast_confirmation_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.read_fast_confirmation_offset()
    }

    fn read_rational_confirmation_offset(
        gateway_id: ChainId,
    ) -> Result<T::BlockNumber, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.read_rational_confirmation_offset()
    }

    fn read_epoch_offset(gateway_id: ChainId) -> Result<T::BlockNumber, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.read_epoch_offset()
    }

    fn verify_event_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.verify_event_inclusion(
            gateway_id,
            message,
            submission_target_height,
        )
    }

    fn verify_state_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.verify_state_inclusion(
            gateway_id,
            message,
            submission_target_height,
        )
    }

    fn verify_tx_inclusion(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
    ) -> Result<Bytes, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.verify_tx_inclusion(
            gateway_id,
            message,
            submission_target_height,
        )
    }

    fn verify_state_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError> {
        let encoded_ingress =
            Self::verify_state_inclusion(gateway_id, message, submission_target_height)?;

        let in_codec = match_vendor_with_codec(
            <T as Config>::Xdns::get_verification_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?,
        );

        recode_bytes_with_descriptor(encoded_ingress, abi_descriptor, in_codec, out_codec)
    }

    fn verify_tx_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError> {
        let encoded_ingress =
            Self::verify_tx_inclusion(gateway_id, message, submission_target_height)?;

        let in_codec = match_vendor_with_codec(
            <T as Config>::Xdns::get_verification_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?,
        );

        recode_bytes_with_descriptor(encoded_ingress, abi_descriptor, in_codec, out_codec)
    }

    fn verify_event_inclusion_and_recode(
        gateway_id: [u8; 4],
        message: Bytes,
        submission_target_height: Option<T::BlockNumber>,
        abi_descriptor: Bytes,
        out_codec: Codec,
    ) -> Result<Bytes, DispatchError> {
        let encoded_ingress =
            Self::verify_event_inclusion(gateway_id, message, submission_target_height)?;

        let in_codec = match_vendor_with_codec(
            <T as Config>::Xdns::get_verification_vendor(&gateway_id)
                .map_err(|_| Error::<T>::GatewayVendorNotFound)?,
        );

        recode_bytes_with_descriptor(encoded_ingress, abi_descriptor, in_codec, out_codec)
    }

    fn initialize(
        origin: OriginFor<T>,
        gateway_id: [u8; 4],
        encoded_registration_data: Bytes,
    ) -> Result<(), DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.initialize(
            origin,
            gateway_id,
            encoded_registration_data,
        )
    }

    fn turn_on(origin: OriginFor<T>, gateway_id: [u8; 4]) -> Result<bool, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.turn_on(origin)
    }

    fn turn_off(origin: OriginFor<T>, gateway_id: [u8; 4]) -> Result<bool, DispatchError> {
        match_light_client_by_gateway_id::<T>(gateway_id)?.turn_off(origin)
    }
}
