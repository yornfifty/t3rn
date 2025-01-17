import Link from "@docusaurus/Link";

export default function IntroCard() {
  return (
    <section className="introCards-main">
      <section className="introCard">
        <section className="introCard-title">Executor</section>
        <section className="introCard-description">
          Executors are off-chain agents acting as market makers that fulfil
          cross-chain orders, earning fees and rewards.
        </section>
        <Link className="introCard-link" to={"executor/executor-overview"}>
          Become an Executor
        </Link>
      </section>
      <section className="introCard">
        <section className="introCard-title">Bridge UI</section>
        <section className="introCard-description">
          Our Bridge UI enables you to make fast, secure, and cost-efficient
          cross-chain transactions.
        </section>
        <Link className="introCard-link" to={"https://bridge.t1rn.io/"}>
          Start Swapping
        </Link>
      </section>
    </section>
  );
}
