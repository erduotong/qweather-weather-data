import os
import pandas as pd


def is_prefecture(ad_code: int) -> bool:
    return ad_code % 100 == 0 and ad_code % 10000 != 0


def is_special(ad_code_str: str) -> bool:
    return ad_code_str.startswith(("71", "81", "82"))


def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    input_csv = os.path.join(script_dir, "China-City-List-latest.csv")
    # 定位表头并使用pandas读取
    with open(input_csv, encoding="utf-8-sig") as f:
        lines = f.readlines()
    header_idx = next(
        (i for i, line in enumerate(lines) if line.startswith("location_id,")), None
    )
    if header_idx is None:
        print("未找到表头")
        return
    df = pd.read_csv(input_csv, header=header_idx, encoding="utf-8-sig")
    # 准备AD_code字段
    df["AD_code_str"] = df["AD_code"].astype(str)
    df["AD_code_int"] = pd.to_numeric(df["AD_code_str"], errors="coerce")
    # 筛选：地级市、直辖市和特别行政区
    mask_pref = (df["AD_code_int"] % 100 == 0) & (df["AD_code_int"] % 10000 != 0)
    # 直辖市 AD_code: 北京110000、天津120000、上海310000、重庆500000
    mask_muni = df["AD_code_int"].isin([110000, 120000, 310000, 500000])
    mask_spec = df["AD_code_str"].str.startswith(("71", "81", "82"))
    df_filtered = df[mask_pref | mask_muni | mask_spec].drop(
        columns=["AD_code_str", "AD_code_int"]
    )
    # 写出结果
    output_dir = os.path.abspath(os.path.join(script_dir, os.pardir, "assets"))
    os.makedirs(output_dir, exist_ok=True)
    output_csv = os.path.join(output_dir, "filtered_cities.csv")
    if not df_filtered.empty:
        df_filtered.to_csv(output_csv, index=False, encoding="utf-8-sig")
    print(f"Filtered {len(df_filtered)} rows to {output_csv}")


if __name__ == "__main__":
    main()
