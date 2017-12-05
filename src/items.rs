use super::item::Item;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Items {
    #[serde(rename = "$value", default)]
    items: Vec<Item>,
}

impl<I> From<I> for Items
where
    I: IntoIterator<Item = Item>,
{
    fn from(items: I) -> Self {
        Items {
            items: items.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs;

    use super::*;
    use super::super::composite::Composite;
    use super::super::spectrogram_result::SpectrogramResult;

    #[test]
    fn deserialize_empty() {
        let input = "<Items/>";
        let expected = Vec::new().into();

        let items: Items = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(items, expected);
    }

    #[test]
    fn deserialize_verbose_empty() {
        let input = "<Items></Items>";
        let expected = Vec::new().into();

        let items: Items = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(items, expected);
    }

    #[test]
    fn deserialize_spectrogram_result() {
        let input =
            "<Items>
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
            </Items>";

        let expected = vec![
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
        ].into();

        let items: Items = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(items, expected);
    }

    #[test]
    fn deserialize_composite() {
        let input =
            "<Items>
                <Composite pid=\"spectrogram_results\" collection=\"t\">
                    <Items>
                        <SpectrogramResult pid=\"ogram\" name=\"Trace\">
                            <TimeSpanStart>-0.011146285714285714</TimeSpanStart>
                            <TimeSpanLength>
                                0.022291428571428573
                            </TimeSpanLength>
                            <RangeFirst>0</RangeFirst>
                            <RangeLast>307</RangeLast>
                            <TriggerLine>1</TriggerLine>
                            <NumLines>317</NumLines>
                            <RefTimestamp>
                                63647134869.552425794857142857
                            </RefTimestamp>
                            <EarliestTS>
                                63647134063.50486790214285714
                            </EarliestTS>
                            <LatestTS>63647134869.54127950914285714</LatestTS>
                            <CurrentRS>8091</CurrentRS>
                            <Lines/>
                        </SpectrogramResult>
                    </Items>
                </Composite>
            </Items>";

        let expected = vec![
            Item::Composite(Composite {
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
            }),
        ].into();

        let items: Items = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(items, expected);
    }

    #[test]
    fn deserialize_spectrogram_results() {
        let input =
            "<Items>
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
                <SpectrogramResult pid=\"ogram\" name=\"Trace\">
                    <TimeSpanStart>-0.02</TimeSpanStart>
                    <TimeSpanLength>0.01</TimeSpanLength>
                    <RangeFirst>1</RangeFirst>
                    <RangeLast>306</RangeLast>
                    <TriggerLine>2</TriggerLine>
                    <NumLines>315</NumLines>
                    <RefTimestamp>1.1</RefTimestamp>
                    <EarliestTS>0.5</EarliestTS>
                    <LatestTS>2.5</LatestTS>
                    <CurrentRS>809</CurrentRS>
                    <Lines/>
                </SpectrogramResult>
            </Items>";

        let expected = vec![
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
            Item::SpectrogramResult(
                SpectrogramResult {
                    pid: "ogram".to_string(),
                    name: "Trace".to_string(),
                    time_span_start: -0.02,
                    time_span_length: 0.01,
                    range_first: 1,
                    range_last: 306,
                    trigger_line: 2,
                    num_lines: 315,
                    ref_timestamp: 1.1,
                    earliest_timestamp: 0.5,
                    latest_timestamp: 2.5,
                    current_rs: 809,
                    lines: vec![].into(),
                }
            ),
        ].into();

        let items: Items = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(items, expected);
    }
}
