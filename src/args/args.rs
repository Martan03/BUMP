use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use pareg::Pareg;

use crate::{core::audio::PlayMsg, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub actions: Vec<PlayMsg>,
    pub ip: IpAddr,
    pub port: u16,
}

impl Args {
    /// Parses cli arguments
    pub fn parse(mut args: Pareg) -> Result<Self, Error> {
        let mut parsed = Args::default();

        while let Some(arg) = args.next() {
            match arg {
                "--ip" => parsed.ip = args.next_arg()?,
                "--port" => parsed.port = args.next_arg()?,
                "-h" | "--help" => Args::help(),
                _ => parsed.parse_msg(&mut args)?,
            }
        }
        Ok(parsed)
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            actions: Default::default(),
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
        }
    }
}

impl Args {
    /// Parses the given message
    fn parse_msg(&mut self, args: &mut Pareg) -> Result<(), Error> {
        let Some(msg) = args.cur() else {
            return Ok(());
        };
        match msg {
            "play" => self.actions.push(PlayMsg::Play(Some(true))),
            "pause" => self.actions.push(PlayMsg::Play(Some(false))),
            "pp" | "playpause" => self.actions.push(PlayMsg::Play(None)),
            "prev" | "p" => self.actions.push(PlayMsg::Prev(None)),
            "next" | "n" => self.actions.push(PlayMsg::Next(None)),
            "mute" => self.actions.push(PlayMsg::Mute),
            "v" | "vol" | "volume" => self.parse_vol(args)?,
            "shuffle" | "mix" => self.actions.push(PlayMsg::Shuffle),
            a => Err(Error::Msg(format!("invalid action: '{a}'")))?,
        }
        Ok(())
    }

    fn parse_vol(&mut self, args: &mut Pareg) -> Result<(), Error> {
        let vol: f32 = args.next_manual(|s| match Args::parse_num(s) {
            Ok(num) if (0.0..1.0).contains(&num) => Ok(num),
            Ok(_) => Err(pareg::ArgError::parse_msg(
                "invalid value range",
                s.to_string(),
            )),
            Err(_) => Err(pareg::ArgError::parse_msg(
                "expected number",
                s.to_string(),
            )),
        })?;
        self.actions.push(PlayMsg::Volume(vol));
        Ok(())
    }

    /// Parses number from the arguments iterator
    fn parse_num<N>(arg: &str) -> Result<N, Error>
    where
        N: FromStr,
    {
        arg.parse::<N>()
            .map_err(|_| Error::Msg(format!("number expected, got '{arg}'")))
    }

    fn help() {
        println!("Not yet done");
    }
}
