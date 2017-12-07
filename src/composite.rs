use super::items::{Items, ItemsSpectrogram};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Composite {
    pub(crate) pid: String,
    pub(crate) collection: String,
    #[serde(rename = "Items")]
    pub(crate) items: Items,
}

impl Composite {
    pub fn data_points<'a>(&'a self) -> ItemsSpectrogram<'a> {
        self.items.data_points()
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs;

    use super::*;
    use super::super::item::Item;
    use super::super::spectrogram_result::SpectrogramResult;

    #[test]
    fn deserialize_spectrogram_result() {
        let input =
            "<Composite pid=\"spectrogram_results\" collection=\"t\">
                <Items>
                    <SpectrogramResult pid=\"ogram\" name=\"Trace\">
                        <TimeSpanStart>-0.011146285714285714</TimeSpanStart>
                        <TimeSpanLength>0.022291428571428573</TimeSpanLength>
                        <RangeFirst>0</RangeFirst>
                        <RangeLast>307</RangeLast>
                        <TriggerLine>1</TriggerLine>
                        <NumLines>317</NumLines>
                        <RefTimestamp>63647134869.552425794857142857</RefTimestamp>
                        <EarliestTS>63647134063.50486790214285714</EarliestTS>
                        <LatestTS>63647134869.54127950914285714</LatestTS>
                        <CurrentRS>8091</CurrentRS>
                        <Lines/>
                    </SpectrogramResult>
                </Items>
            </Composite>";

        let expected = Composite {
            pid: "spectrogram_results".to_string(),
            collection: "t".to_string(),
            items: vec![
                Item::SpectrogramResult(
                    SpectrogramResult {
                        pid: "ogram".to_string(),
                        name: "Trace".to_string(),
                        time_span_start: -0.011146285714285714,
                        time_span_length: 0.022291428571428573,
                        range_first: 0,
                        range_last: 307,
                        trigger_line: 1,
                        num_lines: 317,
                        ref_timestamp: 63647134869.552425794857142857,
                        earliest_timestamp: 63647134063.50486790214285714,
                        latest_timestamp: 63647134869.54127950914285714,
                        current_rs: 8091,
                        lines: vec![].into(),
                    }
                ),
            ].into(),
        };

        let composite: Composite = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(composite, expected);
    }
}
