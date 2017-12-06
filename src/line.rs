use std::str::Chars;

use super::bytes_to_floats::BytesToFloats;
use super::hex_to_byte::HexToByte;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    #[serde(rename = "TS")]
    pub(crate) timestamp: f64,
    #[serde(rename = "RSID")]
    pub(crate) rsid: usize,
    pub(crate) units: String,
    pub(crate) x_start: usize,
    pub(crate) x_range: usize,
    #[serde(rename = "TimeLen")]
    pub(crate) time_length: f64,
    pub(crate) valid: bool,
    #[serde(rename = "RBW")]
    pub(crate) resolution_bandwidth: f64,
    pub(crate) first: bool,
    pub(crate) num_points: usize,
    pub(crate) mult_recs: bool,
    pub(crate) data: String,
}

impl Line {
    pub fn data_points<'a>(&'a self) -> BytesToFloats<HexToByte<Chars<'a>>> {
        let chars = self.data.chars();
        let bytes = HexToByte::new(chars);
        let points = BytesToFloats::new(bytes);

        points
    }
}

#[cfg(test)]
mod tests {
    use serde_xml_rs;

    use super::Line;

    #[test]
    fn deserialize() {
        let input =
            "<Line>
                <TS>63647134063.50486790214285714</TS>
                <RSID>1341</RSID>
                <Units>dBm</Units>
                <XStart>119436695</XStart>
                <XRange>400000</XRange>
                <TimeLen>2.4055942855714285</TimeLen>
                <Valid>true</Valid>
                <RBW>100.00255834564311</RBW>
                <First>true</First>
                <NumPoints>801</NumPoints>
                <MultRecs>false</MultRecs>
                <Data>1A6F02C3EA7102C3D6A001C39A8502C3580202C3DC1703C3DCEA02C38CE002C3295002C37AA502C3DC1703C3865A01C3B95602C38A7702C38BD102C3B29400C3854B01C350E303C35A8902C3D8EB01C31E7D03C3A93902C3029F00C3241501C3CBAF02C3D40A01C39C27FFC2E01900C363C600C31A6002C3B68401C3DE8F03C38D2B03C3D92702C3566C01C303AE00C34C1103C30E6F03C3528B00C3692E02C3380402C3C0DC03C3601200C3326F00C3729800C34BB702C324F700C3E12800C3C2A200C3195102C3CA6402C3128000C31BAB02C35BA702C3565D01C3168E01C3767901C3469A01C33BC4FEC2F7AD01C30E6003C3F4F900C3093402C3B54801C3E4DC00C3F42601C35FC1FFC2B3B200C35A7A02C3F01800C3A53A01C3765B01C3D8DC01C3841E01C3C2A200C3E00A00C3E4DC00C3840F01C3FE5DFFC26E8703C3DCDB02C320E3FFC29E90FFC2593E02C3900F00C3141601C3E29100C378E201C38A5902C3665C01C3C55601C3E0E6FFC2054401C35A9802C35E6A03C3640201C3BA9EFEC2D29200C3702000C30A7F02C3367D01C387D201C37C1AFFC223AC00C3BC16FFC2800100C38A5902C398EF01C327BA01C3C27500C3993A02C3793C02C33C1EFFC2042601C3F3BD00C3C28400C3E3AF00C3101700C3041701C3E3BE00C3E29100C3527C00C3E14600C3FFB7FFC21422FDC2553F01C3DA6FFEC21E98FFC240FFFFC2A92A02C3502200C317B8FDC2B7AEFDC2DE7DFFC273A700C3FCD6FEC2725C00C378FDFDC29FAEFFC2712F00C373A700C320D4FFC240E1FFC2A12C00C3702000C3F4DB00C3F00900C3F541FDC2FE99FFC294F000C3901BFCC237C5FDC2F251FCC26C2DFBC2400500C3DE8CFFC2BBBCFEC21821FEC2165EFDC2E8DBF9C22E7CFBC234D5FCC2165EFDC2940BFDC2069EF9C2A268F8C2E536F9C223CAF8C24FB6FBC220F8F7C25E9AF7C26411F9C272A4FCC23411FDC23012FCC23801FEC2D253FCC277B2FDC23A79FEC22AAAFAC2D7BBFDC29275FCC2DA75F6C238E3FDC2F28DFCC2F65FFDC24E98FBC24EA7FBC2CFCCFBC2B273FCC21FB6FFC2854BF9C2693DFAC207BCF9C25A86FEC2C3C0F8C268E3F9C22A6EFAC22134F8C2C4FCF8C2269CF9C2D6A3F5C28C04F3C2F01BF4C2DE6BEFC25AA1EEC28007F0C2DFB6EFC2313CECC2067DE9C23922E6C22820EAC2E4E8E8C29C1BE7C2952CE5C25D3DE7C25CF2E6C22FBBE3C29A76E6C29FB1E7C2841BE9C22022E8C2841BE9C298EFE5C24020E8C2A8EBE9C268FEE9C28666E9C28E82EBC288DEE9C212A7ECC2B0F8EBC2866FF1C2F28AECC26C0CEBC24E95EBC2C7C5F1C29C24EFC28A9BF2C2C3C6F0C290F4F3C2AAA8F2C2E8F0F1C27413F5C2FE54F7C2ECF8FAC2D40DF5C21682F5C2165EFDC258E1FDC2B0CEFBC2AE65FBC24FC5FBC2F8E6FDC2F01800C37BC0FEC260FDFFC290EEFBC2542DFDC2E12800C372A4FCC2F27200C3D7CAFDC274D1FCC2B427FDC21821FEC250E3FBC2FA9AFEC2C429F9C254D3FCC28E85FBC25E8BF7C2EFBBFBC2EC16FBC25FC7F7C20943FAC2029FF8C2E7CCF9C23003FCC2D144FCC25C13F7C20662F9C2E826FAC222ACF8C2EFBBFBC2652FF9C2381FFEC2E427F9C2880EFAC2CC27FBC26C0FFBC25400FDC2B826FEC2C80AFAC2F822FEC28AA4FAC294DEFCC2D425FDC2EC25FBC2628A00C30E9CFBC21803FEC2501300C32A7DFAC2BA53FEC2582CFEC2D7BBFDC276A3FDC2168BFDC2E13700C3528B00C3901E00C3D670FDC272A4FCC23AA6FEC2F813FEC290EEFBC2B26700C3F822FEC2D543FDC2F414FDC28FC1FBC21821FEC21132FCC2D824FEC2FA4FFEC2D806FEC2FBC7FEC29139FCC26FB4FBC2594AFEC22814FAC230D6FBC29692FDC20FBAFBC25678FDC27D38FFC24CD5FAC292A2FCC2B8F9FDC22A7DFAC230D6FBC273C2FCC2126EFCC24812FAC2681FFAC27C11F7C20D33FBC27E98F7C2D416FDC2466DF9C2781BFEC28BB3FAC2B00AFCC22814FAC270FFFBC24413F9C2F260FCC28E94FBC2B654FDC21812FEC228D8F9C2A81BFAC2164FFDC2D0EAFBC238F2FDC28D3AFBC26EA5FBC2EEACFBC24413F9C2E663F9C2327BFCC20C06FBC22AAAFAC24803FAC29E78F7C23680F5C2E427F9C2A259F8C270FFFBC24005F8C2C3B1F8C270F0FBC2665CF9C2993DF6C203BDF8C2DA57F6C21E71F7C23A70F6C2EBBCFAC2C00CF8C25270F4C223CAF8C276A9F5C2426EF8C2B279F4C21CF9F6C213B0F4C2D40DF5C2DE92F7C20695F1C2B25BF4C2F80AF6C296A7F5C29810F6C25C04F7C2FE9FF7C2FA64F6C2B0F2F3C21C26F7C22AA1F2C26E60F3C2D14AF4C28A50F2C2528EF4C22684F1C2B2ACECC2BA59F6C21818F6C26E6FF3C274E6F4C25814F6C210FCF3C22D37F3C2CFC3F3C2CE96F3C2566FF5C2AFC5F3C272AAF4C2D277F4C20949F2C230FAF3C207B3F1C26680F1C2FE87EFC28252F0C21EA4EFC2FE78EFC2EFB8EBC21CF0EEC2A25FF0C25E64EFC248FAF1C2D7C7EDC2FCE2EEC2B4D9ECC2180FEEC20EA8EBC2E941EAC2B27FECC2C943EAC2A8DCE9C2D7B8EDC25693EDC26A58EAC2CA61EAC26EA2EBC2ED31EBC22820EAC234FFECC283B2E8C2F4F3ECC23D3FE7C268FEE9C2B693E5C264E1E8C208D7E9C2D40AE5C23142E4C2C677E1C2B41BE5C22B9EE2C2C81CE2C28FC4E3C214E9E4C2BD2EDFC2C7A4E1C23E81DFC29940DEC29A5EDEC27E5FDFC2F80DDEC21685DDC23937DEC20698D9C2F12DDCC21921D6C23402D5C2554BD5C22C13D3C24C02D3C27E89CFC2AD3ECBC27CF9C6C29B9DC6C22FB5C3C2E26AC8C2C290C0C2E8F6C1C24BA5C2C2292FC2C2DD53BFC2BCE6C6C22D4CC3C2F520C5C2BD22C7C2CA97CAC28A8CCAC2339FCCC23535CDC2ED49CBC294F3CCC247DCC9C20A85CAC278F4CDC23807CEC26E6FCBC28400D1C24F9ECBC2DBA2CEC29BA6CEC26A97D2C21BDBCEC2E8F9D1C27FA7CFC24D3ED3C2AFB0D3C2F940D6C2B3A0D4C2A80CD2C2593BD6C2B0F5DBC2FD3FD7C24A75DAC2A3ADD8C2BFBBD7C233B1DCC21E92DFC21BC0DEC2FFA2DFC23556DDC243BCE0C227DBE1C2CBD0E2C28E79E3C20538E1C233D8E4C2D927E6C26296E8C250F5E3C23414E5C270EDEBC2253FE9C28D3DE3C298E0E5C2E4D9E8C2B7B1E5C2F934E6C25E79E7C2A546E9C2BFBEE7C2AC26EBC20EA8EBC2494BEAC2B81AE6C29D57E7C28267E8C250D1EBC2AFBCEBC29281ECC2969EEDC243B6E8C2393AEEC29426EDC2DEA7EFC21A96EEC21A69EEC21697EDC2A69AF1C22258F0C2E12EF0C20E8AEBC212A7ECC25AA1EEC20E84F3C218E2EDC27A63EEC28143F0C2EEA3F3C23CD0EEC24827F2C2DEA7EFC292A8F4C2C8D4F1C2CFC3F3C25261F4C25542F5C2001EF0C26825F2C2D0FFF3C25805F6C292A8F4C2728CF4C230FAF3C28932F2C27413F5C2CA97F2C21274F4C26EABF3C2769AF5C2F275F4C23CE8F6C2B25BF4C2040EF1C257BAF5C2C689F1C24664F1C2A0E7EFC29E8DEFC21147F4C2F692F5C2567EF5C2B4D3F4C2CA79F2C228DEF1C2766DF5C298D4F5C27A6CF6C20E75F3C2D259F4C2FBCDF6C280F2F7C25D40F7C264F3F8C2FAA0F6C27014F4C2B7B4F5C25E6DF7C2E273F8C29D4BF7C27D2FF7C2EBCBFAC2BA77F6C2D02CF4C2A6A3F9C28297F8C20C06FBC2C692F9C24014F8C262A8F8C2F674F5C288FFF9C2566FF5C23CD9F6C2E536F9C2A658F9C2127DFCC21014FCC21404FDC21AAEF6C2B0CEFBC22E7CFBC2EE7FFBC2ECE9FAC248E5F9C2F142FCC2A41CF9C2F26FFCC2C8FBF9C2CFAEFBC2A667F9C24A8AFAC210E7FBC2F3C9FCC25687FDC2713E00C3D135FCC29946FEC28CFEFAC263B7F8C2D0F9FBC21BC6FEC254F1FCC29828FEC2381FFEC22BC8FAC29683FDC257C3FDC2F0F7FBC214F5FCC2C819FAC21A99FEC29547FDC2166DFDC25696FDC2F940FEC29A64FEC2167CFDC2FFAEF7C2AEA1FBC2581DFEC2FE4EFFC2A3B300C333BA00C32E4FFBC2DE6EFFC2AC29FBC2B808FEC2729800C3B26700C3D2ADFCC2C0E8FFC2366BFDC2B291FCC244F500C334E4FCC2FA7CFEC21E7AFFC2B146FCC2F67DFDC23A88FEC2514000C344D700C374F200C3415000C3D4EC00C3DFB9FFC2A0DBFFC29538FDC23EA5FFC21250FCC2900CFCC2D3CBFCC29FBDFFC274F200C3602100C3514F00C3B427FDC2514000C31E89FFC29CFAFEC2CA73FAC2A00E00C3F8F5FDC2326000C3E27300C3B409FDC2929600C33BB5FEC233AB00C393C300C303AE00C3BEACFFC2D7AF01C3781BFEC2F15400C3200700C3113500C3FAA9FEC2A29500C3066201C360D0FFC2528B00C3168E01C3026300C3E80802C3FA8BFEC29FAEFFC2BE52FFC2FC12FFC2913C00C39A82FEC260D0FFC23FC3FFC2F66201C3C4FC00C3F4DB00C323CA00C3F29000C3529A00C3C13900C3C41A01C363D500C3D55501C30E51FBC2840F01C3540301C39FCCFFC28EA303C3B67501C300F4FFC2C29300C3467C01C3429B00C318D6FDC2681002C3454F01C3D7CD01C3A94802C3B7C001C3C8FB01C3</Data>
            </Line>";

        let expected = Line {
            timestamp: 63647134063.50486790214285714,
            rsid: 1341,
            units: "dBm".to_string(),
            x_start: 119436695,
            x_range: 400000,
            time_length: 2.4055942855714285,
            valid: true,
            resolution_bandwidth: 100.00255834564311,
            first: true,
            num_points: 801,
            mult_recs: false,
            data: "1A6F02C3EA7102C3D6A001C39A8502C3580202C3DC1703C3DCEA02C38CE002C3295002C37AA502C3DC1703C3865A01C3B95602C38A7702C38BD102C3B29400C3854B01C350E303C35A8902C3D8EB01C31E7D03C3A93902C3029F00C3241501C3CBAF02C3D40A01C39C27FFC2E01900C363C600C31A6002C3B68401C3DE8F03C38D2B03C3D92702C3566C01C303AE00C34C1103C30E6F03C3528B00C3692E02C3380402C3C0DC03C3601200C3326F00C3729800C34BB702C324F700C3E12800C3C2A200C3195102C3CA6402C3128000C31BAB02C35BA702C3565D01C3168E01C3767901C3469A01C33BC4FEC2F7AD01C30E6003C3F4F900C3093402C3B54801C3E4DC00C3F42601C35FC1FFC2B3B200C35A7A02C3F01800C3A53A01C3765B01C3D8DC01C3841E01C3C2A200C3E00A00C3E4DC00C3840F01C3FE5DFFC26E8703C3DCDB02C320E3FFC29E90FFC2593E02C3900F00C3141601C3E29100C378E201C38A5902C3665C01C3C55601C3E0E6FFC2054401C35A9802C35E6A03C3640201C3BA9EFEC2D29200C3702000C30A7F02C3367D01C387D201C37C1AFFC223AC00C3BC16FFC2800100C38A5902C398EF01C327BA01C3C27500C3993A02C3793C02C33C1EFFC2042601C3F3BD00C3C28400C3E3AF00C3101700C3041701C3E3BE00C3E29100C3527C00C3E14600C3FFB7FFC21422FDC2553F01C3DA6FFEC21E98FFC240FFFFC2A92A02C3502200C317B8FDC2B7AEFDC2DE7DFFC273A700C3FCD6FEC2725C00C378FDFDC29FAEFFC2712F00C373A700C320D4FFC240E1FFC2A12C00C3702000C3F4DB00C3F00900C3F541FDC2FE99FFC294F000C3901BFCC237C5FDC2F251FCC26C2DFBC2400500C3DE8CFFC2BBBCFEC21821FEC2165EFDC2E8DBF9C22E7CFBC234D5FCC2165EFDC2940BFDC2069EF9C2A268F8C2E536F9C223CAF8C24FB6FBC220F8F7C25E9AF7C26411F9C272A4FCC23411FDC23012FCC23801FEC2D253FCC277B2FDC23A79FEC22AAAFAC2D7BBFDC29275FCC2DA75F6C238E3FDC2F28DFCC2F65FFDC24E98FBC24EA7FBC2CFCCFBC2B273FCC21FB6FFC2854BF9C2693DFAC207BCF9C25A86FEC2C3C0F8C268E3F9C22A6EFAC22134F8C2C4FCF8C2269CF9C2D6A3F5C28C04F3C2F01BF4C2DE6BEFC25AA1EEC28007F0C2DFB6EFC2313CECC2067DE9C23922E6C22820EAC2E4E8E8C29C1BE7C2952CE5C25D3DE7C25CF2E6C22FBBE3C29A76E6C29FB1E7C2841BE9C22022E8C2841BE9C298EFE5C24020E8C2A8EBE9C268FEE9C28666E9C28E82EBC288DEE9C212A7ECC2B0F8EBC2866FF1C2F28AECC26C0CEBC24E95EBC2C7C5F1C29C24EFC28A9BF2C2C3C6F0C290F4F3C2AAA8F2C2E8F0F1C27413F5C2FE54F7C2ECF8FAC2D40DF5C21682F5C2165EFDC258E1FDC2B0CEFBC2AE65FBC24FC5FBC2F8E6FDC2F01800C37BC0FEC260FDFFC290EEFBC2542DFDC2E12800C372A4FCC2F27200C3D7CAFDC274D1FCC2B427FDC21821FEC250E3FBC2FA9AFEC2C429F9C254D3FCC28E85FBC25E8BF7C2EFBBFBC2EC16FBC25FC7F7C20943FAC2029FF8C2E7CCF9C23003FCC2D144FCC25C13F7C20662F9C2E826FAC222ACF8C2EFBBFBC2652FF9C2381FFEC2E427F9C2880EFAC2CC27FBC26C0FFBC25400FDC2B826FEC2C80AFAC2F822FEC28AA4FAC294DEFCC2D425FDC2EC25FBC2628A00C30E9CFBC21803FEC2501300C32A7DFAC2BA53FEC2582CFEC2D7BBFDC276A3FDC2168BFDC2E13700C3528B00C3901E00C3D670FDC272A4FCC23AA6FEC2F813FEC290EEFBC2B26700C3F822FEC2D543FDC2F414FDC28FC1FBC21821FEC21132FCC2D824FEC2FA4FFEC2D806FEC2FBC7FEC29139FCC26FB4FBC2594AFEC22814FAC230D6FBC29692FDC20FBAFBC25678FDC27D38FFC24CD5FAC292A2FCC2B8F9FDC22A7DFAC230D6FBC273C2FCC2126EFCC24812FAC2681FFAC27C11F7C20D33FBC27E98F7C2D416FDC2466DF9C2781BFEC28BB3FAC2B00AFCC22814FAC270FFFBC24413F9C2F260FCC28E94FBC2B654FDC21812FEC228D8F9C2A81BFAC2164FFDC2D0EAFBC238F2FDC28D3AFBC26EA5FBC2EEACFBC24413F9C2E663F9C2327BFCC20C06FBC22AAAFAC24803FAC29E78F7C23680F5C2E427F9C2A259F8C270FFFBC24005F8C2C3B1F8C270F0FBC2665CF9C2993DF6C203BDF8C2DA57F6C21E71F7C23A70F6C2EBBCFAC2C00CF8C25270F4C223CAF8C276A9F5C2426EF8C2B279F4C21CF9F6C213B0F4C2D40DF5C2DE92F7C20695F1C2B25BF4C2F80AF6C296A7F5C29810F6C25C04F7C2FE9FF7C2FA64F6C2B0F2F3C21C26F7C22AA1F2C26E60F3C2D14AF4C28A50F2C2528EF4C22684F1C2B2ACECC2BA59F6C21818F6C26E6FF3C274E6F4C25814F6C210FCF3C22D37F3C2CFC3F3C2CE96F3C2566FF5C2AFC5F3C272AAF4C2D277F4C20949F2C230FAF3C207B3F1C26680F1C2FE87EFC28252F0C21EA4EFC2FE78EFC2EFB8EBC21CF0EEC2A25FF0C25E64EFC248FAF1C2D7C7EDC2FCE2EEC2B4D9ECC2180FEEC20EA8EBC2E941EAC2B27FECC2C943EAC2A8DCE9C2D7B8EDC25693EDC26A58EAC2CA61EAC26EA2EBC2ED31EBC22820EAC234FFECC283B2E8C2F4F3ECC23D3FE7C268FEE9C2B693E5C264E1E8C208D7E9C2D40AE5C23142E4C2C677E1C2B41BE5C22B9EE2C2C81CE2C28FC4E3C214E9E4C2BD2EDFC2C7A4E1C23E81DFC29940DEC29A5EDEC27E5FDFC2F80DDEC21685DDC23937DEC20698D9C2F12DDCC21921D6C23402D5C2554BD5C22C13D3C24C02D3C27E89CFC2AD3ECBC27CF9C6C29B9DC6C22FB5C3C2E26AC8C2C290C0C2E8F6C1C24BA5C2C2292FC2C2DD53BFC2BCE6C6C22D4CC3C2F520C5C2BD22C7C2CA97CAC28A8CCAC2339FCCC23535CDC2ED49CBC294F3CCC247DCC9C20A85CAC278F4CDC23807CEC26E6FCBC28400D1C24F9ECBC2DBA2CEC29BA6CEC26A97D2C21BDBCEC2E8F9D1C27FA7CFC24D3ED3C2AFB0D3C2F940D6C2B3A0D4C2A80CD2C2593BD6C2B0F5DBC2FD3FD7C24A75DAC2A3ADD8C2BFBBD7C233B1DCC21E92DFC21BC0DEC2FFA2DFC23556DDC243BCE0C227DBE1C2CBD0E2C28E79E3C20538E1C233D8E4C2D927E6C26296E8C250F5E3C23414E5C270EDEBC2253FE9C28D3DE3C298E0E5C2E4D9E8C2B7B1E5C2F934E6C25E79E7C2A546E9C2BFBEE7C2AC26EBC20EA8EBC2494BEAC2B81AE6C29D57E7C28267E8C250D1EBC2AFBCEBC29281ECC2969EEDC243B6E8C2393AEEC29426EDC2DEA7EFC21A96EEC21A69EEC21697EDC2A69AF1C22258F0C2E12EF0C20E8AEBC212A7ECC25AA1EEC20E84F3C218E2EDC27A63EEC28143F0C2EEA3F3C23CD0EEC24827F2C2DEA7EFC292A8F4C2C8D4F1C2CFC3F3C25261F4C25542F5C2001EF0C26825F2C2D0FFF3C25805F6C292A8F4C2728CF4C230FAF3C28932F2C27413F5C2CA97F2C21274F4C26EABF3C2769AF5C2F275F4C23CE8F6C2B25BF4C2040EF1C257BAF5C2C689F1C24664F1C2A0E7EFC29E8DEFC21147F4C2F692F5C2567EF5C2B4D3F4C2CA79F2C228DEF1C2766DF5C298D4F5C27A6CF6C20E75F3C2D259F4C2FBCDF6C280F2F7C25D40F7C264F3F8C2FAA0F6C27014F4C2B7B4F5C25E6DF7C2E273F8C29D4BF7C27D2FF7C2EBCBFAC2BA77F6C2D02CF4C2A6A3F9C28297F8C20C06FBC2C692F9C24014F8C262A8F8C2F674F5C288FFF9C2566FF5C23CD9F6C2E536F9C2A658F9C2127DFCC21014FCC21404FDC21AAEF6C2B0CEFBC22E7CFBC2EE7FFBC2ECE9FAC248E5F9C2F142FCC2A41CF9C2F26FFCC2C8FBF9C2CFAEFBC2A667F9C24A8AFAC210E7FBC2F3C9FCC25687FDC2713E00C3D135FCC29946FEC28CFEFAC263B7F8C2D0F9FBC21BC6FEC254F1FCC29828FEC2381FFEC22BC8FAC29683FDC257C3FDC2F0F7FBC214F5FCC2C819FAC21A99FEC29547FDC2166DFDC25696FDC2F940FEC29A64FEC2167CFDC2FFAEF7C2AEA1FBC2581DFEC2FE4EFFC2A3B300C333BA00C32E4FFBC2DE6EFFC2AC29FBC2B808FEC2729800C3B26700C3D2ADFCC2C0E8FFC2366BFDC2B291FCC244F500C334E4FCC2FA7CFEC21E7AFFC2B146FCC2F67DFDC23A88FEC2514000C344D700C374F200C3415000C3D4EC00C3DFB9FFC2A0DBFFC29538FDC23EA5FFC21250FCC2900CFCC2D3CBFCC29FBDFFC274F200C3602100C3514F00C3B427FDC2514000C31E89FFC29CFAFEC2CA73FAC2A00E00C3F8F5FDC2326000C3E27300C3B409FDC2929600C33BB5FEC233AB00C393C300C303AE00C3BEACFFC2D7AF01C3781BFEC2F15400C3200700C3113500C3FAA9FEC2A29500C3066201C360D0FFC2528B00C3168E01C3026300C3E80802C3FA8BFEC29FAEFFC2BE52FFC2FC12FFC2913C00C39A82FEC260D0FFC23FC3FFC2F66201C3C4FC00C3F4DB00C323CA00C3F29000C3529A00C3C13900C3C41A01C363D500C3D55501C30E51FBC2840F01C3540301C39FCCFFC28EA303C3B67501C300F4FFC2C29300C3467C01C3429B00C318D6FDC2681002C3454F01C3D7CD01C3A94802C3B7C001C3C8FB01C3".to_string(),
        };

        let line: Line = serde_xml_rs::from_str(input).unwrap();

        assert_eq!(line, expected);
    }

    #[test]
    fn points() {
        let line = Line {
            timestamp: 0.0,
            rsid: 0,
            units: "dBm".to_string(),
            x_start: 0,
            x_range: 0,
            time_length: 0.0,
            valid: false,
            resolution_bandwidth: 0.0,
            first: false,
            num_points: 4,
            mult_recs: false,
            data: "0000803f000080bf0000003f0000c0bf".to_string(),
        };

        let points: Vec<f32> = line.data_points().collect();
        let expected = vec![1.0, -1.0, 0.5, -1.5];

        assert_eq!(points, expected);
    }
}
