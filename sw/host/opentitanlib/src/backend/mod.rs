// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use thiserror::Error;

use crate::app::config::process_config_file;
use crate::app::{TransportWrapper, TransportWrapperBuilder};
use crate::transport::hyperdebug::{C2d2Flavor, CW310Flavor, StandardFlavor, Ti50Flavor};
use crate::transport::{EmptyTransport, Transport};
use crate::util::parse_int::ParseInt;

mod cw310;
mod hyperdebug;
mod proxy;
mod ti50emulator;
mod ultradebug;
mod verilator;

#[derive(Debug, StructOpt)]
pub struct BackendOpts {
    #[structopt(long, default_value = "", help = "Name of the debug interface")]
    pub interface: String,

    #[structopt(long, parse(try_from_str = u16::from_str),
                help="USB Vendor ID of the interface")]
    pub usb_vid: Option<u16>,
    #[structopt(long, parse(try_from_str = u16::from_str),
                help="USB Product ID of the interface")]
    pub usb_pid: Option<u16>,
    #[structopt(long, help = "USB serial number of the interface")]
    pub usb_serial: Option<String>,

    #[structopt(flatten)]
    pub cw310_opts: cw310::Cw310Opts,

    #[structopt(flatten)]
    pub verilator_opts: verilator::VerilatorOpts,

    #[structopt(flatten)]
    pub proxy_opts: proxy::ProxyOpts,

    #[structopt(flatten)]
    pub ti50emulator_opts: ti50emulator::Ti50EmulatorOpts,

    #[structopt(long, number_of_values(1), help = "Configuration files")]
    pub conf: Vec<PathBuf>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown interface {0}")]
    UnknownInterface(String),
}

/// Creates the requested backend interface according to [`BackendOpts`].
pub fn create(args: &BackendOpts) -> Result<TransportWrapper> {
    let interface = args.interface.as_str();
    let mut env = TransportWrapperBuilder::new(interface.to_string());

    for conf_file in &args.conf {
        process_config_file(&mut env, conf_file.as_ref())?
    }
    let (backend, default_conf) = match env.get_interface() {
        "" => (create_empty_transport()?, None),
        "proxy" => (proxy::create(&args.proxy_opts)?, None),
        "verilator" => (
            verilator::create(&args.verilator_opts)?,
            Some(Path::new("/__builtin__/opentitan_verilator.json")),
        ),
        "ti50emulator" => (
            ti50emulator::create(&args.ti50emulator_opts)?,
            Some(Path::new("/__builtin__/ti50emulator.json")),
        ),
        "ultradebug" => (
            ultradebug::create(args)?,
            Some(Path::new("/__builtin__/opentitan_ultradebug.json")),
        ),
        "hyper310" => (
            hyperdebug::create::<CW310Flavor>(args)?,
            Some(Path::new("/__builtin__/hyperdebug_cw310.json")),
        ),
        "hyperdebug" => (hyperdebug::create::<StandardFlavor>(args)?, None),
        "hyperdebug_dfu" => (hyperdebug::create_dfu(args)?, None),
        "c2d2" => (
            hyperdebug::create::<C2d2Flavor>(args)?,
            Some(Path::new("/__builtin__/h1dx_devboard.json")),
        ),
        "ti50" => (hyperdebug::create::<Ti50Flavor>(args)?, None),
        "cw310" => (
            cw310::create(args)?,
            Some(Path::new("/__builtin__/opentitan_cw310.json")),
        ),
        _ => return Err(Error::UnknownInterface(interface.to_string()).into()),
    };
    if args.conf.is_empty() {
        if let Some(conf_file) = default_conf {
            process_config_file(&mut env, conf_file)?
        }
    }
    env.build(backend)
}

pub fn create_empty_transport() -> Result<Box<dyn Transport>> {
    Ok(Box::new(EmptyTransport))
}
