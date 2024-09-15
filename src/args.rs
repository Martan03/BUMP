use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use crate::{core::audio::PlayMsg, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub actions: Vec<PlayMsg>,
    pub ip: IpAddr,
    pub port: u16,
}

impl Args {
    /// Parses cli arguments
    pub fn parse(args: std::env::Args) -> Result<Self, Error> {
        let mut parsed = Args::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "--ip" => parsed.parse_ip(&mut args_iter)?,
                "--port" => parsed.port = Self::parse_num(&mut args_iter)?,
                "-h" | "--help" => Args::help(),
                msg => parsed.parse_msg(&mut args_iter, &msg)?,
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
    fn parse_msg<T>(&mut self, args: &mut T, msg: &str) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        match msg {
            "play" => self.actions.push(PlayMsg::Play),
            "pause" => self.actions.push(PlayMsg::Pause),
            "pp" | "playpause" => self.actions.push(PlayMsg::PlayPause),
            "prev" | "p" => self.actions.push(PlayMsg::Prev),
            "next" | "n" => self.actions.push(PlayMsg::Next),
            "mute" => self.actions.push(PlayMsg::Mute),
            "v" | "vol" | "volume" => self.parse_vol(args)?,
            "shuffle" | "mix" => self.actions.push(PlayMsg::Shuffle),
            a => Err(Error::Msg(format!("invalid action: '{a}'")))?,
        }
        Ok(())
    }

    /// Parses ip address from the arguments iterator
    fn parse_ip<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        let Some(arg) = args.next() else {
            return Err(Error::Msg(
                "IP address expected after `--ip`".to_string(),
            ));
        };
        self.ip =
            IpAddr::from_str(&arg).map_err(|e| Error::Msg(format!("{e}")))?;
        Ok(())
    }

    fn parse_vol<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        let vol = Args::parse_num(args)?;
        if !(0.0..1.0).contains(&vol) {
            Err(Error::Msg(
                "volume expects value between 0 and 1".to_string(),
            ))?
        }
        self.actions.push(PlayMsg::Volume(vol));
        Ok(())
    }

    /// Parses number from the arguments iterator
    fn parse_num<N, T>(args: &mut T) -> Result<N, Error>
    where
        N: FromStr,
        T: Iterator<Item = String>,
    {
        let Some(val) = args.next() else {
            return Err(Error::Msg("missing argument parameter".into()));
        };

        val.parse::<N>()
            .map_err(|_| Error::Msg(format!("number expected, got '{val}'")))
    }

    fn help() {
        println!("Not yet done");
    }
}
