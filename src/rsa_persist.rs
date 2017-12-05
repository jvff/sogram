use super::rsa_persist_internal::RsaPersistInternal;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RsaPersist {
    #[serde(rename = "version")]
    version: String,
    application: String,
    internal: RsaPersistInternal,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs;

    use super::*;
    use super::super::composite::Composite;
    use super::super::item::Item;
    use super::super::spectrogram_result::SpectrogramResult;

    #[test]
    fn deserialize() {
        let input =
            "<RSAPersist version=\"1.12\">
                <Application>RSA6100</Application>
                <Internal>
                    <Composite pid=\"spectrogram_results\" collection=\"t\">
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
                    </Composite>
                </Internal>
            </RSAPersist>";

        let expected = RsaPersist {
            version: "1.12".to_string(),
            application: "RSA6100".to_string(),
            internal: RsaPersistInternal {
                composite: Composite {
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
                },
            },
        };

        let rsa_persist: RsaPersist = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(rsa_persist, expected);
    }
}
