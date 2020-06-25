use chrono::prelude::*;

struct Utilisateur
{
    nom: String,
    date: Date<Utc>,
}

fn main()
{
    println!("COUCOU")
}

fn give_bonus(utilisateur: &Utilisateur, today: Date<Utc>) -> u64
{
    if today - utilisateur.date >= chrono::Duration::days(365*2)
    {
        return 90;
    }
    else if today - utilisateur.date >= chrono::Duration::days(365)
    {
        return 95;
    }
    100
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_facto()
    {
        let ex = Utilisateur
        {
            nom: String::from("Alexandre"),
            date: Utc.ymd(2018, 7, 8),
        };

        assert_eq!(give_bonus(&ex, Utc.ymd(2019, 7, 8)), 95);
        assert_eq!(give_bonus(&ex, Utc.ymd(2018, 7, 9)), 100);
        assert_eq!(give_bonus(&ex, Utc.ymd(2020, 7, 8)), 90);
    }
}