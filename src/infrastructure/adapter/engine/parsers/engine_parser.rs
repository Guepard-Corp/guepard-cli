use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "infrastructure/adapter/engine/parsers/zfs.pest"] // relative to src
pub struct EngineParser;

#[derive(Debug, PartialEq)]
pub struct Volume {
    pub name: String,
    pub used: u64,
    pub avail: u64,
    pub size: u64,
    pub refer: u64,
    pub mountpoint: String,
}

pub fn parse_zfs_list(input: &str) -> Vec<Volume> {
    assert!(input.len() > 0, "Input must not be empty");

    let mut volumes = Vec::new();
    let pairs = EngineParser::parse(Rule::zfs_list, input).expect("Failed to parse");

    for pair in pairs {
        if pair.as_rule() == Rule::volume_line {
            volumes.push(parse_volume(pair));
        }
    }

    volumes
}

fn parse_volume(pair: pest::iterators::Pair<Rule>) -> Volume {
    let mut volume = Volume {
        name: String::new(),
        used: 0,
        avail: 0,
        size: 0,
        refer: 0,
        mountpoint: String::new(),
    };


    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::name => volume.name = inner_pair.as_str().trim().to_string(),
            Rule::used => volume.used = inner_pair.as_str().trim().to_string().parse::<u64>().unwrap(),
            Rule::avail => volume.avail = inner_pair.as_str().trim().to_string().parse::<u64>().unwrap_or(0),
            Rule::refer => volume.refer = inner_pair.as_str().trim().to_string().parse::<u64>().unwrap(),
            Rule::mountpoint => volume.mountpoint = inner_pair.as_str().trim().to_string(),
            _ => unreachable!(),
        }
    }

    volume.size = volume.used + volume.avail;
    volume
}

#[cfg(test)]
mod test {
    use super::{parse_zfs_list, Volume};

    #[test]
    fn test_parse_zfs_list() {
        let input = r#"NAME         USED  AVAIL  REFER  MOUNTPOINT
tank        1024  100  100  /Volumes/tank
tank/test   1024  100  100  /Volumes/tank/test
tank/test2  1024  100  100  /Volumes/tank/test2
tank/test3  1024  100  100  /Volumes/tank/test3
tank_ext/mongodb-dev-1    1024  100  100  /Volumes/tank_ext/mongodb-dev-1"#;

        let expected_volumes = vec![
            Volume {
                name: "tank".to_string(),
                used: 1024,
                avail: 100,
                refer: 100,
                size: 1124,
                mountpoint: "/Volumes/tank".to_string(),
            },
            Volume {
                name: "tank/test".to_string(),
                used: 1024,
                avail: 100,
                size: 1124,
                refer: 100,
                mountpoint: "/Volumes/tank/test".to_string(),
            },
            Volume {
                name: "tank/test2".to_string(),
                used: 1024,
                avail: 100,
                size: 1124,
                refer: 100,
                mountpoint: "/Volumes/tank/test2".to_string(),
            },
            Volume {
                name: "tank/test3".to_string(),
                used: 1024,
                avail: 100,
                size: 1124,
                refer: 100,
                mountpoint: "/Volumes/tank/test3".to_string(),
            },
            Volume {
                name: "tank_ext/mongodb-dev-1".to_string(),
                used: 1024,
                avail: 100,
                size: 1124,
                refer: 100,
                mountpoint: "/Volumes/tank_ext/mongodb-dev-1".to_string(),
            },
        ];

        let parsed_volumes = parse_zfs_list(input);

        assert_eq!(parsed_volumes, expected_volumes);
    }

    #[test]
    fn test_parse_zfs_list_snapshots() {
        let input = r#"NAME         USED  AVAIL  REFER  MOUNTPOINT
tank        0  -  100 -"#;

        let expected_volumes = vec![
            Volume {
                name: "tank".to_string(),
                used: 0,
                avail: 0,
                refer: 100,
                size: 0,
                mountpoint: "-".to_string(),
            },
        ];

        let parsed_volumes = parse_zfs_list(input);

        assert_eq!(parsed_volumes, expected_volumes);
    }
}