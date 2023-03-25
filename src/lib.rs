use std::collections::HashMap;

pub fn format_row(key: String, value: String, max: &u32) -> String {
    let max = *max;
    let height: f64 = value.parse::<u32>().unwrap() as f64 / max as f64;
    let row_template = r#"<tr>
            <th scope="row"><span class="drac-text drac-text-cyan"> <!--  {row_key}--></span></th>
            <td style="--size:<!--  {row_height}-->;"> <span class="data"> <span
                    class="drac-text drac-text-cyan"><!--  {row_data}--></span> </span> <span class="tooltip">data: <!--  {row_data}--></span>
            </td>
            <!-- {next_data}-->
        </tr>"#;

    row_template
        .replace(r"<!--  {row_key}-->", key.as_str())
        .replace(r"<!--  {row_data}-->", value.as_str())
        .replace(r"<!--  {row_height}-->", height.to_string().as_str())
        .replace(r"<!-- {next_data}-->", r#"<!--  {data}-->"#)
}

pub fn table_start() -> String {
    r#"<table id="animated-bar" class="charts-css column show-labels show-data-on-hover show-primary-axis" style="height:200px;">
        <thead>
            <tr>
                <th>data</th>
            </tr>
        </thead>
        <tbody>
        <!--  {data}-->"#
        .to_string()
}

pub fn table_end() -> String {
    r#"</tbody>
    </table>"#
        .to_string()
}

pub fn div_start(heading: String) -> String {
    r#"<div class="drac-border-cyan" style="border-style: solid; border-radius: 25px; border-width: 1px;">
    <div class="drac-box drac-p-md">
    <h2 class="drac-heading drac-heading-xl drac-text-cyan">
      <!--  {heading}-->
    </h2>
    <!--  {data}-->
    "#.replace(r"<!--  {heading}-->", heading.as_str())
}

pub fn sort_data(data: &HashMap<String, (u32, u32)>) -> Vec<(String, u32, u32)> {
    let mut data = data.clone();
    let mut mut_data = {
        let mut ret = Vec::new();
        for (key, (order, value)) in data.iter_mut() {
            ret.push((key.clone(), *order, *value));
        }
        ret
    };

    mut_data.sort_by(|(_, order1, _), (_, order2, _)| order1.cmp(order2));

    mut_data.clone()
}
