// Copyright 2025 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// automatically generated by tools/bindgen.sh

#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    non_snake_case,
    clippy::ptr_as_ptr,
    clippy::undocumented_unsafe_blocks,
    missing_debug_implementations,
    clippy::tests_outside_test_module,
    unsafe_op_in_unsafe_fn
)]

pub const VIRTIO_BLK_F_SIZE_MAX: u32 = 1;
pub const VIRTIO_BLK_F_SEG_MAX: u32 = 2;
pub const VIRTIO_BLK_F_GEOMETRY: u32 = 4;
pub const VIRTIO_BLK_F_RO: u32 = 5;
pub const VIRTIO_BLK_F_BLK_SIZE: u32 = 6;
pub const VIRTIO_BLK_F_TOPOLOGY: u32 = 10;
pub const VIRTIO_BLK_F_MQ: u32 = 12;
pub const VIRTIO_BLK_F_DISCARD: u32 = 13;
pub const VIRTIO_BLK_F_WRITE_ZEROES: u32 = 14;
pub const VIRTIO_BLK_F_SECURE_ERASE: u32 = 16;
pub const VIRTIO_BLK_F_ZONED: u32 = 17;
pub const VIRTIO_BLK_F_BARRIER: u32 = 0;
pub const VIRTIO_BLK_F_SCSI: u32 = 7;
pub const VIRTIO_BLK_F_FLUSH: u32 = 9;
pub const VIRTIO_BLK_F_CONFIG_WCE: u32 = 11;
pub const VIRTIO_BLK_F_WCE: u32 = 9;
pub const VIRTIO_BLK_ID_BYTES: u32 = 20;
pub const VIRTIO_BLK_T_IN: u32 = 0;
pub const VIRTIO_BLK_T_OUT: u32 = 1;
pub const VIRTIO_BLK_T_SCSI_CMD: u32 = 2;
pub const VIRTIO_BLK_T_FLUSH: u32 = 4;
pub const VIRTIO_BLK_T_GET_ID: u32 = 8;
pub const VIRTIO_BLK_T_DISCARD: u32 = 11;
pub const VIRTIO_BLK_T_WRITE_ZEROES: u32 = 13;
pub const VIRTIO_BLK_T_SECURE_ERASE: u32 = 14;
pub const VIRTIO_BLK_T_ZONE_APPEND: u32 = 15;
pub const VIRTIO_BLK_T_ZONE_REPORT: u32 = 16;
pub const VIRTIO_BLK_T_ZONE_OPEN: u32 = 18;
pub const VIRTIO_BLK_T_ZONE_CLOSE: u32 = 20;
pub const VIRTIO_BLK_T_ZONE_FINISH: u32 = 22;
pub const VIRTIO_BLK_T_ZONE_RESET: u32 = 24;
pub const VIRTIO_BLK_T_ZONE_RESET_ALL: u32 = 26;
pub const VIRTIO_BLK_T_BARRIER: u32 = 2147483648;
pub const VIRTIO_BLK_Z_NONE: u32 = 0;
pub const VIRTIO_BLK_Z_HM: u32 = 1;
pub const VIRTIO_BLK_Z_HA: u32 = 2;
pub const VIRTIO_BLK_ZT_CONV: u32 = 1;
pub const VIRTIO_BLK_ZT_SWR: u32 = 2;
pub const VIRTIO_BLK_ZT_SWP: u32 = 3;
pub const VIRTIO_BLK_ZS_NOT_WP: u32 = 0;
pub const VIRTIO_BLK_ZS_EMPTY: u32 = 1;
pub const VIRTIO_BLK_ZS_IOPEN: u32 = 2;
pub const VIRTIO_BLK_ZS_EOPEN: u32 = 3;
pub const VIRTIO_BLK_ZS_CLOSED: u32 = 4;
pub const VIRTIO_BLK_ZS_RDONLY: u32 = 13;
pub const VIRTIO_BLK_ZS_FULL: u32 = 14;
pub const VIRTIO_BLK_ZS_OFFLINE: u32 = 15;
pub const VIRTIO_BLK_WRITE_ZEROES_FLAG_UNMAP: u32 = 1;
pub const VIRTIO_BLK_S_OK: u32 = 0;
pub const VIRTIO_BLK_S_IOERR: u32 = 1;
pub const VIRTIO_BLK_S_UNSUPP: u32 = 2;
pub const VIRTIO_BLK_S_ZONE_INVALID_CMD: u32 = 3;
pub const VIRTIO_BLK_S_ZONE_UNALIGNED_WP: u32 = 4;
pub const VIRTIO_BLK_S_ZONE_OPEN_RESOURCE: u32 = 5;
pub const VIRTIO_BLK_S_ZONE_ACTIVE_RESOURCE: u32 = 6;
