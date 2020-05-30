mod players_tests {
    use crate::types::Players;
    use crate::events::Card;

    fn simple_setup() -> Players {
        let mut players = Players::with_capacity(4);
        players.insert_at_tail("p1".to_string(), Card::Guard);
        players.insert_at_tail("p2".to_string(), Card::Priest);
        players.insert_at_tail("p3".to_string(), Card::Baron);

        players
    }

    #[test]
    fn simple_increment_turn() {
        let mut players = simple_setup();

        assert_eq!("p1", players.current_turn_player_id());
        players.increment_turn();
        assert_eq!("p2", players.current_turn_player_id());
        players.increment_turn();
        assert_eq!("p3", players.current_turn_player_id());
        players.increment_turn();
        assert_eq!("p1", players.current_turn_player_id());
    }

    #[test]
    fn eliminate_and_increment_turn_chooses_next_player_correctly() {
        // P1 turn, eliminate all players
        {
            let mut players = simple_setup();

            assert_eq!("p1", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p1");
            assert_eq!("p2", players.current_turn_player_id());
        }
        {
            let mut players = simple_setup();

            assert_eq!("p1", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p2");
            assert_eq!("p3", players.current_turn_player_id());
        }
        {
            let mut players = simple_setup();

            assert_eq!("p1", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p3");
            assert_eq!("p2", players.current_turn_player_id());
        }

        // P2 turn, eliminate all players
        {
            let mut players = simple_setup();
            players.increment_turn();

            assert_eq!("p2", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p1");
            assert_eq!("p3", players.current_turn_player_id());
        }
        {
            let mut players = simple_setup();
            players.increment_turn();

            assert_eq!("p2", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p2");
            assert_eq!("p3", players.current_turn_player_id());
        }
        {
            let mut players = simple_setup();
            players.increment_turn();

            assert_eq!("p2", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p3");
            assert_eq!("p1", players.current_turn_player_id());
        }

        // P3 turn, eliminate all players
        {
            let mut players = simple_setup();
            players.increment_turn();
            players.increment_turn();

            assert_eq!("p3", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p1");
            assert_eq!("p2", players.current_turn_player_id());
        }
        {
            let mut players = simple_setup();
            players.increment_turn();
            players.increment_turn();

            assert_eq!("p3", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p2");
            assert_eq!("p1", players.current_turn_player_id());
        }
        {
            let mut players = simple_setup();
            players.increment_turn();
            players.increment_turn();

            assert_eq!("p3", players.current_turn_player_id());
            players.eliminate_and_increment_turn("p3");
            assert_eq!("p1", players.current_turn_player_id());
        }
    }

    #[test]
    fn eliminate_and_increment_turn_returns_eliminated_card() {
        let mut players = simple_setup();
        assert_eq!(Card::Priest, players.eliminate_and_increment_turn("p2"));
        assert_eq!(Card::Guard, players.eliminate_and_increment_turn("p1"));
        assert_eq!(Card::Baron, players.eliminate_and_increment_turn("p3"));
    }
}
