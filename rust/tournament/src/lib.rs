use std::collections::HashMap;

struct Stat {
    pub played: i16,
    pub wins: i16,
    pub draws: i16,
    pub losses: i16,
    pub points: i16,
}

impl Stat {
    pub fn new() -> Self {
        Self {
            played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    pub fn add_win(&mut self) {
        self.played += 1;
        self.wins += 1;
        self.points += 3;
    }

    pub fn add_loss(&mut self) {
        self.played += 1;
        self.losses += 1;
    }

    pub fn add_draw(&mut self) {
        self.played += 1;
        self.draws += 1;
        self.points += 1;
    }

    pub fn to_string(&self) -> String {
        format!("| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                self.played, self.wins, self.draws, self.losses, self.points)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Stat> = HashMap::new();
    let mut table = ["Team                           | MP |  W |  D |  L |  P".to_string()].to_vec();
    let matches = match_results.trim()
        .split('\n');

    for game in matches {
        if game.is_empty() {
            continue;
        }
        let info: Vec<String> = game.split(";")
            .map(|s| s.to_string())
            .collect();
        match info[2].as_str() {
            "win" => {
                teams.entry(info[0].clone())
                    .or_insert(Stat::new())
                    .add_win();
                teams.entry(info[1].clone())
                    .or_insert(Stat::new())
                    .add_loss();
            }
            "loss" => {
                teams.entry(info[0].clone())
                    .or_insert(Stat::new())
                    .add_loss();
                teams.entry(info[1].clone())
                    .or_insert(Stat::new())
                    .add_win();
            }
            "draw" => {
                teams.entry(info[0].clone())
                    .or_insert(Stat::new())
                    .add_draw();
                teams.entry(info[1].clone())
                    .or_insert(Stat::new())
                    .add_draw();
            }
            _ => unreachable!(),
        }
    }

    let mut team_names: Vec<String> = teams.keys()
        .cloned()
        .collect();
    team_names.sort_by_key(|t| (teams.get(t).unwrap().points * -1, t.to_string()));
    for team in team_names {
        let i = teams.get(&team)
            .unwrap();
        table.push(format!("{:31}{}",
                           team, i.to_string()));
    }
    table.join("\n")
        .to_string()
}
