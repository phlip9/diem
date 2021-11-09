// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{AdvertisedData, GlobalDataSummary, OptimalChunkSizes};
use diem_config::network_id::PeerNetworkId;
use std::collections::HashMap;
use storage_service_types::{StorageServerSummary, StorageServiceRequest};

#[derive(Debug, Default)]
struct PeerState {
    storage_summary: Option<StorageServerSummary>,
    // TODO(philiphayes): imagine storing some scoring info here.
    metadata: (),
}

/// Contains all of the unbanned peers' most recent [`StorageServerSummary`] data
/// advertisements and data-client internal metadata for scoring.
#[derive(Debug)]
pub(crate) struct PeerStates {
    inner: HashMap<PeerNetworkId, PeerState>,
}

impl PeerStates {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Returns true if a connected storage service peer can actually fulfill a
    /// request, given our current view of their advertised data summary.
    pub fn can_service_request(
        &self,
        peer: &PeerNetworkId,
        request: &StorageServiceRequest,
    ) -> bool {
        // Storage services can always respond to data advertisement requests.
        // We need this outer check, since we need to be able to send data summary
        // requests to new peers (who don't have a peer state yet).
        if request.is_get_storage_server_summary() {
            return true;
        }

        self.inner
            .get(peer)
            .and_then(|peer_state| peer_state.storage_summary.as_ref())
            .map(|summary| summary.can_service(request))
            .unwrap_or(false)
    }

    pub fn update_summary(&mut self, peer: PeerNetworkId, summary: StorageServerSummary) {
        self.inner.entry(peer).or_default().storage_summary = Some(summary);
    }

    pub fn aggregate_summary(&self) -> GlobalDataSummary {
        let mut aggregate_data = AdvertisedData::empty();

        let mut max_epoch_chunk_sizes = vec![];
        let mut max_transaction_chunk_sizes = vec![];
        let mut max_transaction_output_chunk_sizes = vec![];
        let mut max_account_states_chunk_sizes = vec![];

        let summaries = self
            .inner
            .values()
            .filter_map(|state| state.storage_summary.as_ref());

        // collect each peer's protocol and data advertisements
        for summary in summaries {
            // collect aggregate data advertisements
            if let Some(account_states) = summary.data_summary.account_states {
                aggregate_data.account_states.push(account_states);
            }
            if let Some(epoch_ending_ledger_infos) = summary.data_summary.epoch_ending_ledger_infos
            {
                aggregate_data
                    .epoch_ending_ledger_infos
                    .push(epoch_ending_ledger_infos);
            }
            if let Some(synced_ledger_info) = summary.data_summary.synced_ledger_info.as_ref() {
                aggregate_data
                    .synced_ledger_infos
                    .push(synced_ledger_info.clone());
            }
            if let Some(transactions) = summary.data_summary.transactions {
                aggregate_data.transactions.push(transactions);
            }
            if let Some(transaction_outputs) = summary.data_summary.transaction_outputs {
                aggregate_data.transaction_outputs.push(transaction_outputs);
            }

            // collect preferred max chunk sizes
            max_epoch_chunk_sizes.push(summary.protocol_metadata.max_epoch_chunk_size);
            max_transaction_chunk_sizes.push(summary.protocol_metadata.max_transaction_chunk_size);
            max_transaction_output_chunk_sizes
                .push(summary.protocol_metadata.max_transaction_output_chunk_size);
            max_account_states_chunk_sizes
                .push(summary.protocol_metadata.max_account_states_chunk_size);
        }

        // take the median for each max chunk size parameter.
        // this works well when we have an honest majority that mostly agrees on
        // the same chunk sizes.
        // TODO(philiphayes): move these constants somewhere?
        let aggregate_chunk_sizes = OptimalChunkSizes {
            account_states_chunk_size: median(&mut max_account_states_chunk_sizes)
                .unwrap_or(storage_service_server::MAX_ACCOUNT_STATES_CHUNK_SIZE),
            epoch_chunk_size: median(&mut max_epoch_chunk_sizes)
                .unwrap_or(storage_service_server::MAX_EPOCH_CHUNK_SIZE),
            transaction_chunk_size: median(&mut max_transaction_chunk_sizes)
                .unwrap_or(storage_service_server::MAX_TRANSACTION_CHUNK_SIZE),
            transaction_output_chunk_size: median(&mut max_transaction_output_chunk_sizes)
                .unwrap_or(storage_service_server::MAX_TRANSACTION_OUTPUT_CHUNK_SIZE),
        };

        GlobalDataSummary {
            advertised_data: aggregate_data,
            optimal_chunk_sizes: aggregate_chunk_sizes,
        }
    }
}

fn median<T: Ord + Copy>(values: &mut [T]) -> Option<T> {
    values.sort_unstable();
    let idx = values.len() / 2;
    values.get(idx).copied()
}
