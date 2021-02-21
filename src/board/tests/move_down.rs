#[cfg(test)]
mod move_down {
    use crate::board::tests::*;

    mod can_perform {
        use super::*;
        #[test]
        fn test_cyan() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("cyan");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 3, y: 1 },
                    Cell { x: 4, y: 1 },
                    Cell { x: 5, y: 1 },
                    Cell { x: 6, y: 1 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }

        #[test]
        fn test_blue() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("blue");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 3, y: 1 },
                    Cell { x: 3, y: 2 },
                    Cell { x: 4, y: 2 },
                    Cell { x: 5, y: 2 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }

        #[test]
        fn test_orange() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("orange");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 3, y: 2 },
                    Cell { x: 4, y: 2 },
                    Cell { x: 5, y: 1 },
                    Cell { x: 5, y: 2 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }

        #[test]
        fn test_green() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("green");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 3, y: 2 },
                    Cell { x: 4, y: 1 },
                    Cell { x: 4, y: 2 },
                    Cell { x: 5, y: 1 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }

        #[test]
        fn test_red() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("red");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 3, y: 1 },
                    Cell { x: 4, y: 1 },
                    Cell { x: 4, y: 2 },
                    Cell { x: 5, y: 2 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }

        #[test]
        fn test_purple() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("purple");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 3, y: 2 },
                    Cell { x: 4, y: 1 },
                    Cell { x: 4, y: 2 },
                    Cell { x: 5, y: 2 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }

        #[test]
        fn test_yellow() {
            let mut color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block("yellow");
            let mut board = Board::from_data(&color_state, &block, None);
            board.draw_block();
            board.move_down();

            fill_cells(
                &mut color_state,
                &vec![
                    Cell { x: 4, y: 1 },
                    Cell { x: 4, y: 2 },
                    Cell { x: 5, y: 1 },
                    Cell { x: 5, y: 2 },
                ],
            );
            assert!(equals(&blocks_to_fill_color(&board.state), &color_state));
        }
    }

    mod cannot_perform {
        use super::*;

        fn test_cannot_perform_bottom(block_color: &str, rect: Rect) {
            let color_state = from_char_to_color(&EMPTY_BOARD.clone());
            let block = get_block(block_color);
            let mut board = Board::from_data(&color_state, &block, Some(rect));

            board.draw_block();
            let state_before = board.state.clone();
            board.erase_block();
            board.move_down();
            board.draw_block();

            assert!(equals(
                &blocks_to_fill_color(&state_before),
                &blocks_to_fill_color(&board.state)
            ));
        }
        #[test]
        fn test_cyan() {
            test_cannot_perform_bottom(
                "cyan",
                Rect {
                    x: 3,
                    y: 19,
                    width: 4,
                    height: 1,
                },
            );
        }

        #[test]
        fn test_blue() {
            test_cannot_perform_bottom(
                "blue",
                Rect {
                    x: 3,
                    y: 18,
                    width: 3,
                    height: 2,
                },
            );
        }

        #[test]
        fn test_orange() {
            test_cannot_perform_bottom(
                "orange",
                Rect {
                    x: 3,
                    y: 18,
                    width: 3,
                    height: 2,
                },
            );
        }

        #[test]
        fn test_green() {
            test_cannot_perform_bottom(
                "green",
                Rect {
                    x: 3,
                    y: 18,
                    width: 3,
                    height: 2,
                },
            );
        }

        #[test]
        fn test_red() {
            test_cannot_perform_bottom(
                "red",
                Rect {
                    x: 3,
                    y: 18,
                    width: 3,
                    height: 2,
                },
            );
        }

        #[test]
        fn test_purple() {
            test_cannot_perform_bottom(
                "purple",
                Rect {
                    x: 3,
                    y: 18,
                    width: 3,
                    height: 2,
                },
            );
        }

        #[test]
        fn test_yellow() {
            test_cannot_perform_bottom(
                "yellow",
                Rect {
                    x: 4,
                    y: 18,
                    width: 2,
                    height: 2,
                },
            );
        }
    }
}
