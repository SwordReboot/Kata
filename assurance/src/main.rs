use chrono::prelude::*;

struct Utilisateur
{
    nom: String,
    date: Date<Utc>,
    accident: Vec<Date<Utc>>,
}

fn main()
{
    println!("COUCOU")
}

fn give_bonus(utilisateur: &Utilisateur, today: Date<Utc>) -> i64
{
    let annee = (today - utilisateur.date).num_days()/365;
    let mut bonus = 100 - annee * 5;
    if utilisateur.accident.len() != 0
    {
        bonus *= 125;
        bonus /= 100;
    }
    bonus
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_bonus()
    {
        let ex = Utilisateur
        {
            nom: String::from("Alexandre"),
            date: Utc.ymd(2018, 7, 8),
            accident: Vec::new(),
        };

        assert_eq!(give_bonus(&ex, Utc.ymd(2019, 7, 8)), 95);   // un an apres
        assert_eq!(give_bonus(&ex, Utc.ymd(2018, 7, 9)), 100);  // demain
        assert_eq!(give_bonus(&ex, Utc.ymd(2020, 7, 8)), 90);   // deux ans apres
        assert_eq!(give_bonus(&ex, Utc.ymd(2021, 7, 8)), 85);   // trois ans apres
    }

    #[test]
    fn test_malus()
    {
        let mut ex = Utilisateur
        {
            nom: String::from("Jacques"),
            date: Utc.ymd(2018, 7, 8),
            accident: Vec::new(),
        };
        ex.accident.push(Utc.ymd(2018, 7, 9));
        assert_eq!(give_bonus(&ex, Utc.ymd(2018, 7, 10)), 125);

        //ex.accident.push(Utc.ymd(2020, 7, 9));
        assert_eq!(give_bonus(&ex, Utc.ymd(2019, 7, 10)), 125);
    }

    #[test]
    fn test_yvan()
    {
        let mut ex = Utilisateur
        {
            nom: String::from("Yvan"),
            date: Utc.ymd(2018, 7, 8),
            accident: Vec::new(),
        };
        assert_eq!(give_bonus(&ex, Utc.ymd(2022, 7, 8)), 80);
        ex.accident.push(Utc.ymd(2022, 7, 9));
        assert_eq!(give_bonus(&ex, Utc.ymd(2022, 7, 10)), 100);
    }
}