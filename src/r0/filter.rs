//! Endpoints for event filters.

pub mod create_filter;
pub mod get_filter;

use js_int::UInt;
use ruma_identifiers::{RoomId, UserId};
use serde::{Deserialize, Serialize};

/// Format to use for returned events
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum EventFormat {
    /// Client format, as described in the Client API.
    #[serde(rename = "client")]
    Client,
    /// Raw events from federation.
    #[serde(rename = "federation")]
    Federation,
}

/// Filters to be applied to room events
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoomEventFilter {
    /// A list of event types to exclude.
    ///
    /// If this list is absent then no event types are excluded. A matching type will be excluded
    /// even if it is listed in the 'types' filter. A '*' can be used as a wildcard to match any
    /// sequence of characters.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub not_types: Vec<String>,
    /// A list of room IDs to exclude.
    ///
    /// If this list is absent then no rooms are excluded. A matching room will be excluded even if
    /// it is listed in the 'rooms' filter.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub not_rooms: Vec<String>,
    /// The maximum number of events to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<UInt>,
    /// A list of room IDs to include.
    ///
    /// If this list is absent then all rooms are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rooms: Option<Vec<RoomId>>,
    /// A list of sender IDs to exclude.
    ///
    /// If this list is absent then no senders are excluded. A matching sender will be excluded even
    /// if it is listed in the 'senders' filter.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub not_senders: Vec<UserId>,
    /// A list of senders IDs to include.
    ///
    /// If this list is absent then all senders are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub senders: Option<Vec<UserId>>,
    /// A list of event types to include.
    ///
    /// If this list is absent then all event types are included. A '*' can be used as a wildcard to
    /// match any sequence of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub types: Option<Vec<String>>,
}

impl RoomEventFilter {
    /// A filter that can be used to ignore all room events
    pub fn ignore_all() -> Self {
        Self {
            limit: Some(UInt::from(0u32)),
            ..Default::default()
        }
    }
}

/// Filters to be applied to room data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RoomFilter {
    /// Include rooms that the user has left in the sync.
    ///
    /// Defaults to false if not included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_leave: Option<bool>,
    /// The per user account data to include for rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_data: Option<RoomEventFilter>,
    /// The message and state update events to include for rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<RoomEventFilter>,
    /// The events that aren't recorded in the room history, e.g. typing and receipts, to include
    /// for rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<RoomEventFilter>,
    /// The state events to include for rooms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<RoomEventFilter>,
    /// A list of room IDs to exclude.
    ///
    /// If this list is absent then no rooms are excluded. A matching room will be excluded even if
    /// it is listed in the 'rooms' filter. This filter is applied before the filters in
    /// `ephemeral`, `state`, `timeline` or `account_data`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub not_rooms: Vec<RoomId>,
    /// A list of room IDs to include.
    ///
    /// If this list is absent then all rooms are included. This filter is applied before the
    /// filters in `ephemeral`, `state`, `timeline` or `account_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rooms: Option<Vec<RoomId>>,
}

impl RoomFilter {
    /// A filter that can be used to ignore all room events (of any type)
    pub fn ignore_all() -> Self {
        Self {
            account_data: Some(RoomEventFilter::ignore_all()),
            timeline: Some(RoomEventFilter::ignore_all()),
            ephemeral: Some(RoomEventFilter::ignore_all()),
            state: Some(RoomEventFilter::ignore_all()),
            ..Default::default()
        }
    }
}

/// Filter for not-room data
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Filter {
    /// A list of event types to exclude.
    ///
    /// If this list is absent then no event types are excluded. A matching type will be excluded
    /// even if it is listed in the 'types' filter. A '*' can be used as a wildcard to match any
    /// sequence of characters.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub not_types: Vec<String>,
    /// The maximum number of events to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<UInt>,
    /// A list of senders IDs to include.
    ///
    /// If this list is absent then all senders are included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub senders: Option<Vec<UserId>>,
    /// A list of event types to include.
    ///
    /// If this list is absent then all event types are included. A '*' can be used as a wildcard to
    /// match any sequence of characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub types: Option<Vec<String>>,
    /// A list of sender IDs to exclude.
    ///
    /// If this list is absent then no senders are excluded. A matching sender will be excluded even
    /// if it is listed in the 'senders' filter.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub not_senders: Vec<UserId>,
}

impl Filter {
    /// A filter that can be used to ignore all events
    pub fn ignore_all() -> Self {
        Self {
            limit: Some(UInt::from(0u32)),
            ..Default::default()
        }
    }
}

/// A filter definition
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FilterDefinition {
    /// List of event fields to include.
    ///
    /// If this list is absent then all fields are included. The entries may include '.' charaters
    /// to indicate sub-fields. So ['content.body'] will include the 'body' field of the 'content'
    /// object. A literal '.' character in a field name may be escaped using a '\'. A server may
    /// include more fields than were requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub event_fields: Option<Vec<String>>,
    /// The format to use for events.
    ///
    /// 'client' will return the events in a format suitable for clients. 'federation' will return
    /// the raw event as receieved over federation. The default is 'client'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_format: Option<EventFormat>,
    /// The user account data that isn't associated with rooms to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_data: Option<Filter>,
    /// Filters to be applied to room data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<RoomFilter>,
    /// The presence updates to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Filter>,
}

impl FilterDefinition {
    /// A filter that can be used to ignore all events
    pub fn ignore_all() -> Self {
        Self {
            account_data: Some(Filter::ignore_all()),
            room: Some(RoomFilter::ignore_all()),
            presence: Some(Filter::ignore_all()),
            ..Default::default()
        }
    }
}
