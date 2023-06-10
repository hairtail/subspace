(function() {var implementors = {
"domain_service":[["impl&lt;Block, SBlock, PBlock, Client, SClient&gt; PreValidateTransaction for <a class=\"struct\" href=\"domain_service/struct.CoreDomainTxPreValidator.html\" title=\"struct domain_service::CoreDomainTxPreValidator\">CoreDomainTxPreValidator</a>&lt;Block, SBlock, PBlock, Client, SClient&gt;<span class=\"where fmt-newline\">where\n    Block: BlockT,\n    SBlock: BlockT,\n    PBlock: BlockT,\n    SBlock::Hash: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Block::Hash&gt;,\n    NumberFor&lt;SBlock&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;NumberFor&lt;Block&gt;&gt;,\n    Client: ProvideRuntimeApi&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    Client::Api: MessengerApi&lt;Block, NumberFor&lt;Block&gt;&gt;,\n    SClient: HeaderBackend&lt;SBlock&gt; + ProvideRuntimeApi&lt;SBlock&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    SClient::Api: MessengerApi&lt;SBlock, NumberFor&lt;SBlock&gt;&gt; + SettlementApi&lt;SBlock, Block::Hash&gt;,</span>"]],
"subspace_service":[["impl&lt;Block, Client, Verifier, BundleValidator&gt; PreValidateTransaction for <a class=\"struct\" href=\"subspace_service/tx_pre_validator/struct.PrimaryChainTxPreValidator.html\" title=\"struct subspace_service::tx_pre_validator::PrimaryChainTxPreValidator\">PrimaryChainTxPreValidator</a>&lt;Block, Client, Verifier, BundleValidator&gt;<span class=\"where fmt-newline\">where\n    Block: BlockT,\n    Client: ProvideRuntimeApi&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,\n    Client::Api: PreValidationObjectApi&lt;Block, Hash&gt;,\n    Verifier: VerifyFraudProof&lt;Block&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,\n    BundleValidator: ValidateBundle&lt;Block, Hash&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()