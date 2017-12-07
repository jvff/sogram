use super::line::LineSpectrogram;
use super::lines::LinesSpectrogram;

pub enum ItemSpectrogram<'a> {
    SpectrogramResult(LinesSpectrogram<'a>),
    Composite(Box<Iterator<Item = LineSpectrogram<'a>> + 'a>),
}

impl<'a> Iterator for ItemSpectrogram<'a> {
    type Item = LineSpectrogram<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            ItemSpectrogram::SpectrogramResult(ref mut lines) => lines.next(),
            ItemSpectrogram::Composite(ref mut items) => items.next(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::line::Line;
    use super::super::lines::Lines;

    #[test]
    fn spectrogram_result() {
        let lines: Lines = vec![
            Line {
                timestamp: 0.0,
                rsid: 0,
                units: "dBm".to_string(),
                x_start: 0,
                x_range: 0,
                time_length: 1.0,
                valid: false,
                resolution_bandwidth: 0.0,
                first: false,
                num_points: 3,
                mult_recs: false,
                data: "000000bf0000803e9a9919bf".to_string(),
            },
            Line {
                timestamp: 1.0,
                rsid: 0,
                units: "dBm".to_string(),
                x_start: 0,
                x_range: 0,
                time_length: 1.0,
                valid: false,
                resolution_bandwidth: 0.0,
                first: false,
                num_points: 3,
                mult_recs: false,
                data: "000000000000403fcdccccbd".to_string(),
            },
            Line {
                timestamp: 2.0,
                rsid: 0,
                units: "dBm".to_string(),
                x_start: 0,
                x_range: 0,
                time_length: 1.0,
                valid: false,
                resolution_bandwidth: 0.0,
                first: false,
                num_points: 3,
                mult_recs: false,
                data: "0000003f0000a03fcdcccc3e".to_string(),
            },
        ].into();

        let lines_spectrogram = lines.data_points();
        let spectrogram = ItemSpectrogram::SpectrogramResult(lines_spectrogram);

        let expected = vec![
            vec![-0.5, 0.25, -0.6],
            vec![0.0, 0.75, -0.1],
            vec![0.5, 1.25, 0.4],
        ];

        let result: Vec<Vec<f32>> =
            spectrogram.map(|points| points.collect()).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn composite_result() {
        let lines: Lines = vec![
            Line {
                timestamp: 0.0,
                rsid: 0,
                units: "dBm".to_string(),
                x_start: 0,
                x_range: 0,
                time_length: 1.0,
                valid: false,
                resolution_bandwidth: 0.0,
                first: false,
                num_points: 3,
                mult_recs: false,
                data: "000000bf0000803e9a9919bf".to_string(),
            },
            Line {
                timestamp: 1.0,
                rsid: 0,
                units: "dBm".to_string(),
                x_start: 0,
                x_range: 0,
                time_length: 1.0,
                valid: false,
                resolution_bandwidth: 0.0,
                first: false,
                num_points: 3,
                mult_recs: false,
                data: "000000000000403fcdccccbd".to_string(),
            },
            Line {
                timestamp: 2.0,
                rsid: 0,
                units: "dBm".to_string(),
                x_start: 0,
                x_range: 0,
                time_length: 1.0,
                valid: false,
                resolution_bandwidth: 0.0,
                first: false,
                num_points: 3,
                mult_recs: false,
                data: "0000003f0000a03fcdcccc3e".to_string(),
            },
        ].into();

        let lines_spectrogram = lines.data_points();
        let boxed_spectrogram = Box::new(lines_spectrogram);
        let spectrogram = ItemSpectrogram::Composite(boxed_spectrogram);

        let expected = vec![
            vec![-0.5, 0.25, -0.6],
            vec![0.0, 0.75, -0.1],
            vec![0.5, 1.25, 0.4],
        ];

        let result: Vec<Vec<f32>> =
            spectrogram.map(|points| points.collect()).collect();

        assert_eq!(result, expected);
    }
}
