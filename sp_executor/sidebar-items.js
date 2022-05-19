initSidebarItems({"enum":[["ExecutionPhase","Execution phase along with an optional encoded call data."],["VerificationError","Error type of fraud proof verification on primary node."]],"mod":[["fraud_proof_ext",""]],"struct":[["Bundle","Transaction bundle"],["BundleEquivocationProof","Represents a bundle equivocation proof. An equivocation happens when an executor produces more than one bundle on the same slot. The proof of equivocation are the given distinct bundle headers that were signed by the validator and which include the slot number."],["BundleHeader","Header of transaction bundle."],["ExecutionReceipt","Receipt of state execution."],["ExecutorKey","A type that implements `BoundToRuntimeAppPublic`, used for executor signing key."],["FraudProof","Fraud proof for the state computation."],["InvalidTransactionProof","Represents an invalid transaction proof."],["OpaqueBundle","Bundle with opaque extrinsics."],["SignedBundle","Signed version of [`Bundle`]."],["SignedExecutionReceipt","Signed version of [`ExecutionReceipt`] which will be gossiped over the executors network."],["SignedOpaqueBundle","Signed version of [`OpaqueBundle`]."]],"trait":[["ExecutorApi","API necessary for executor pallet."]],"type":[["ExecutorId","An executor authority identifier."],["ExecutorPair","An executor authority keypair. Necessarily equivalent to the schnorrkel public key used in the main executor module. If that ever changes, then this must, too."],["ExecutorSignature","An executor authority signature."]]});