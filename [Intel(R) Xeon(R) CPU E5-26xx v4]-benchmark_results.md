# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-11-21 13:18:53
## Hardware Information
- CPU: Intel(R) Xeon(R) CPU E5-26xx v4 (2 cores)
- Memory: 3787 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex          68079 | ntex          68847 | may_minihttp     71007 |
|    2 | may_minihttp     66976 | may_minihttp     66410 | ntex          68738 |
|    3 | ohkami        64144 | ohkami        61083 | ohkami        68402 |
|    4 | xitca-web     59919 | actix-web     60522 | actix-web     63232 |
|    5 | actix-web     57171 | xitca-web     58733 | xitca-web     60073 |
|    6 | warp          55865 | warp          53716 | warp          57289 |
|    7 | argan         51535 | argan         52215 | viz           52361 |
|    8 | viz           51468 | axum          51489 | argan         52201 |
|    9 | axum          50584 | iron          50953 | poem          51167 |
|   10 | poem          48196 | viz           50848 | graphul       50575 |
|   11 | gotham        46862 | poem          48201 | axum          50212 |
|   12 | graphul       45152 | gotham        47414 | salvo         49614 |
|   13 | salvo         45142 | graphul       47133 | gotham        48635 |
|   14 | iron          42006 | salvo         45388 | iron          47499 |
|   15 | silent        39954 | silent        38821 | silent        40838 |
|   16 | rocket        33470 | nickel        32892 | rocket        34363 |
|   17 | nickel        32422 | rocket        30634 | nickel        34193 |
|   18 | oxidy         15467 | oxidy         15706 | oxidy         16721 |
|   19 | summer-boot      1431 | summer-boot      1436 | summer-boot      1434 |
|   20 | tide           1430 | tide           1430 | tide           1433 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex          64190 | ntex          66317 | may_minihttp     68817 |
|    2 | ohkami        62386 | may_minihttp     63536 | ntex          65825 |
|    3 | may_minihttp     61965 | ohkami        62979 | actix-web     64669 |
|    4 | actix-web     57646 | actix-web     60792 | ohkami        62662 |
|    5 | xitca-web     54364 | xitca-web     56019 | xitca-web     56783 |
|    6 | argan         52710 | warp          50748 | argan         54408 |
|    7 | warp          51945 | argan         48926 | warp          54080 |
|    8 | axum          46389 | viz           48494 | viz           50344 |
|    9 | poem          43369 | axum          45236 | graphul       49236 |
|   10 | salvo         42822 | graphul       44201 | axum          47403 |
|   11 | graphul       42564 | poem          43788 | gotham        45913 |
|   12 | gotham        41895 | salvo         42683 | salvo         45031 |
|   13 | viz           41305 | gotham        40611 | poem          44881 |
|   14 | iron          39211 | iron          35718 | silent        38754 |
|   15 | silent        35800 | silent        34806 | iron          37159 |
|   16 | nickel        31350 | nickel        32235 | nickel        34037 |
|   17 | rocket        29022 | rocket        29676 | rocket        30093 |
|   18 | oxidy         15556 | oxidy         15468 | oxidy         16171 |
|   19 | summer-boot      5681 | tide           5669 | summer-boot      5680 |
|   20 | tide           5638 | summer-boot      5662 | tide           5657 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ntex          58992 | may_minihttp     63536 | may_minihttp     67348 |
|    2 | may_minihttp     57898 | ntex          59411 | ntex          61519 |
|    3 | ohkami        56179 | ohkami        56363 | actix-web     61084 |
|    4 | actix-web     55356 | actix-web     55699 | ohkami        60539 |
|    5 | xitca-web     51873 | xitca-web     54205 | warp          52769 |
|    6 | warp          48895 | warp          49043 | xitca-web     52018 |
|    7 | argan         47995 | argan         48647 | argan         51261 |
|    8 | viz           44104 | axum          45126 | axum          48683 |
|    9 | graphul       43894 | viz           43366 | viz           46148 |
|   10 | axum          42592 | gotham        41862 | graphul       45659 |
|   11 | poem          41343 | graphul       41333 | gotham        45019 |
|   12 | gotham        40468 | salvo         41313 | salvo         43524 |
|   13 | salvo         39864 | poem          39314 | poem          43231 |
|   14 | iron          33549 | silent        34551 | iron          34990 |
|   15 | silent        33182 | iron          33585 | silent        34985 |
|   16 | nickel        31618 | nickel        33489 | nickel        34089 |
|   17 | rocket        28815 | rocket        28506 | rocket        27833 |
|   18 | oxidy         15097 | oxidy         15460 | oxidy         15721 |
|   19 | tide          10787 | tide          10923 | summer-boot     10869 |
|   20 | summer-boot     10720 | summer-boot     10854 | tide          10852 |
