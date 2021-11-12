initSidebarItems({"mod":[["accumulator","This module implements an in-memory Merkle Accumulator that is similar to what we use in storage. This accumulator will only store a small portion of the tree – for any subtree that is full, we store only the root. Also we only store the frozen nodes, therefore this structure will always store up to `Log(n)` number of nodes, where `n` is the total number of leaves in the tree."],["default_protocol",""],["definition","This module has definition of various proofs."],["position","This module provides an abstraction for positioning a node in a binary tree, A `Position` uniquely identifies the location of a node"],["proptest_proof","All proofs generated in this module are not valid proofs. They are only for the purpose of testing conversion between Rust and Protobuf."]],"struct":[["AccountStateProof","The complete proof used to authenticate the state of an account. This structure consists of the `AccumulatorProof` from `LedgerInfo` to `TransactionInfo`, the `TransactionInfo` object and the `SparseMerkleProof` from state root to the account."],["AccumulatorConsistencyProof","A proof that can be used to show that two Merkle accumulators are consistent – the big one can be obtained by appending certain leaves to the small one. For example, at some point in time a client knows that the root hash of the ledger at version 10 is `old_root` (it could be a waypoint). If a server wants to prove that the new ledger at version `N` is derived from the old ledger the client knows, it can show the subtrees that represent all the new leaves. If the client can verify that it can indeed obtain the new root hash by appending these new leaves, it can be convinced that the two accumulators are consistent."],["AccumulatorExtensionProof","A proof that first verifies that establishes correct computation of the root and then returns the new tree to acquire a new root and version."],["AccumulatorProof","A proof that can be used authenticate an element in an accumulator given trusted root hash. For example, both `LedgerInfoToTransactionInfoProof` and `TransactionInfoToEventProof` can be constructed on top of this structure."],["AccumulatorRangeProof","A proof that is similar to `AccumulatorProof`, but can be used to authenticate a range of leaves. For example, given the following accumulator:"],["EventProof","The complete proof used to authenticate a contract event. This structure consists of the `AccumulatorProof` from `LedgerInfo` to `TransactionInfo`, the `TransactionInfo` object and the `AccumulatorProof` from event accumulator root to the event."],["MerkleTreeInternalNode",""],["SparseMerkleLeafNode",""],["SparseMerkleLeafNodeHasher","Cryptographic hasher for an BCS-serializable #item"],["SparseMerkleProof","A proof that can be used to authenticate an element in a Sparse Merkle Tree given trusted root hash. For example, `TransactionInfoToAccountProof` can be constructed on top of this structure."],["SparseMerkleRangeProof","Note: this is not a range proof in the sense that a range of nodes is verified! Instead, it verifies the entire left part of the tree up to a known rightmost node. See the description below."],["TransactionAccumulatorSummary","An in-memory accumulator for storing a summary of the core transaction info accumulator. It is a summary in the sense that it only stores maximally frozen subtree nodes rather than storing all leaves and internal nodes."],["TransactionInfoListWithProof","The proof used to authenticate a list of consecutive transaction infos."],["TransactionInfoWithProof","`TransactionInfo` and a `TransactionAccumulatorProof` connecting it to the ledger root."]],"type":[["EventAccumulatorInternalNode",""],["EventAccumulatorProof",""],["SparseMerkleInternalNode",""],["TestAccumulatorInternalNode",""],["TestAccumulatorProof",""],["TestAccumulatorRangeProof",""],["TransactionAccumulatorInternalNode",""],["TransactionAccumulatorProof",""],["TransactionAccumulatorRangeProof",""]]});