// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// tb__xbar_connect generated by `tlgen.py` tool


xbar_aon dut(
  // TODO temp use same clk to avoid failure due to new feature (multi-clk #903)
  .clk_aon_i(clk),
  .rst_aon_ni(rst_n)
);

// Host TileLink interface connections
`CONNECT_TL_HOST_IF(main)

// Device TileLink interface connections
`CONNECT_TL_DEVICE_IF(pwrmgr_aon)
`CONNECT_TL_DEVICE_IF(rstmgr_aon)
`CONNECT_TL_DEVICE_IF(clkmgr_aon)
`CONNECT_TL_DEVICE_IF(rbox_aon)
`CONNECT_TL_DEVICE_IF(pwm_aon)
`CONNECT_TL_DEVICE_IF(pinmux_aon)
`CONNECT_TL_DEVICE_IF(padctrl_aon)
`CONNECT_TL_DEVICE_IF(timer_aon)
`CONNECT_TL_DEVICE_IF(usbdev_aon)
`CONNECT_TL_DEVICE_IF(ram_ret_aon)
