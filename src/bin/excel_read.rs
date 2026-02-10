use calamine::{Reader, Xlsx, open_workbook};
use indexmap::IndexSet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 打开 Excel 文件
    let path = "/Users/wenzhou/Downloads/2026-02-11工单数据.xlsx"; // 替换为你的文件路径
    // let mut excel: ExcelReader<_> = ExcelReader::from_path(path)?;
    let mut excel: Xlsx<_> = open_workbook(path)?;

    // 2. 获取工作表名称列表 (可选，用于调试)
    let sheet_names = excel.sheet_names();
    println!("工作表列表: {:?}", sheet_names);

    // 3. 读取第一个工作表的数据
    // 注意：如果文件很大，这里会将整个表格加载到内存中
    let range = excel.worksheet_range_at(0).ok_or("无法读取工作表")?;

    let mut data_set: IndexSet<String> = IndexSet::new();
    //4. 遍历并打印数据
    for row in range?.rows() {
        for cell in row {
            data_set.insert(cell.to_string());
        }
        break;
    }
    println!("{:?}", data_set);
    println!("{}", ((240 - 90) * 60 - 1500) * 24);
    Ok(())
}
