# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-17 16:13:34
## Hardware Information
- CPU:  (8 cores)
- Memory: 16033 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web   1079132 | actix-web   1017347 | xitca-web   1141487 |
|    2 | actix-web    960112 | xitca-web   1005223 | actix-web   1059663 |
|    3 | ntex         923018 | ntex         971276 | ntex        1009232 |
|    4 | iron         645026 | iron         672999 | iron         673672 |
|    5 | warp         443665 | warp         472426 | ohkami       507568 |
|    6 | viz          442345 | ohkami       455625 | warp         497622 |
|    7 | ohkami       440646 | graphul      436395 | argan        481414 |
|    8 | axum         435106 | axum         428534 | axum         465091 |
|    9 | graphul      432373 | thruster     421290 | thruster     450860 |
|   10 | thruster     430480 | salvo        420072 | graphul      448546 |
|   11 | argan        406487 | argan        419998 | poem         445934 |
|   12 | astra        403378 | viz          419312 | salvo        422668 |
|   13 | gotham       401233 | poem         405348 | gotham       412431 |
|   14 | poem         399547 | astra        396755 | viz          407215 |
|   15 | saphir       374680 | saphir       379083 | rocket       380651 |
|   16 | salvo        374569 | gotham       378206 | astra        378312 |
|   17 | rocket       361238 | rocket       373808 | saphir       371823 |
|   18 | silent       319302 | silent       319507 | trillium     328304 |
|   19 | trillium     301356 | nickel       318528 | silent       327799 |
|   20 | nickel       256401 | trillium     314578 | nickel       280310 |
|   21 | oxidy        123125 | oxidy        118630 | oxidy        146394 |
|   22 | rouille       63640 | rouille       65087 | rouille       63035 |
|   23 | tide           1513 | tide           1512 | summer-boot      1501 |
|   24 | summer-boot      1505 | summer-boot      1506 | tide           1499 |
|   25 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | xitca-web   1151449 | actix-web   1159568 | xitca-web   1234694 |
|    2 | actix-web   1121658 | ntex        1131823 | actix-web   1220567 |
|    3 | ntex        1081695 | xitca-web   1112463 | ntex        1196971 |
|    4 | warp         806213 | warp         838431 | warp         869638 |
|    5 | ohkami       755397 | ohkami       762674 | axum         793812 |
|    6 | viz          742910 | thruster     738175 | ohkami       790568 |
|    7 | thruster     724401 | viz          735448 | argan        779600 |
|    8 | axum         714756 | argan        727089 | viz          775127 |
|    9 | graphul      702034 | axum         724746 | graphul      765147 |
|   10 | gotham       682929 | gotham       694496 | thruster     754589 |
|   11 | argan        680955 | poem         657262 | gotham       715652 |
|   12 | saphir       654973 | salvo        649952 | saphir       699098 |
|   13 | poem         641332 | saphir       639693 | salvo        691636 |
|   14 | salvo        635407 | iron         615772 | poem         686151 |
|   15 | trillium     495319 | graphul      569193 | trillium     520792 |
|   16 | rocket       426926 | trillium     481241 | iron         509902 |
|   17 | astra        362987 | rocket       434412 | rocket       455856 |
|   18 | iron         355829 | astra        368106 | astra        367241 |
|   19 | silent       351262 | silent       356491 | silent       367065 |
|   20 | nickel       304745 | nickel       265243 | nickel       351679 |
|   21 | oxidy        137432 | oxidy        124493 | oxidy        155657 |
|   22 | rouille       63517 | rouille       57469 | rouille       61503 |
|   23 | tide           6107 | summer-boot      6125 | summer-boot      6115 |
|   24 | summer-boot      6044 | tide           5952 | tide           5981 |
|   25 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web   1175044 | actix-web   1251556 | actix-web   1304308 |
|    2 | xitca-web   1156895 | ntex        1188067 | xitca-web   1283981 |
|    3 | ntex        1129741 | xitca-web   1150410 | ntex        1253943 |
|    4 | ohkami       906715 | ohkami       945724 | warp         966940 |
|    5 | warp         869455 | warp         899906 | ohkami       945787 |
|    6 | thruster     831963 | thruster     859859 | thruster     878498 |
|    7 | argan        807950 | argan        845167 | viz          869331 |
|    8 | viz          788623 | viz          786020 | argan        863927 |
|    9 | axum         766991 | axum         772485 | axum         860962 |
|   10 | graphul      745503 | graphul      763574 | graphul      810840 |
|   11 | gotham       727798 | gotham       753844 | gotham       793233 |
|   12 | saphir       704745 | poem         692474 | saphir       755211 |
|   13 | poem         676641 | salvo        678246 | salvo        717625 |
|   14 | salvo        664866 | saphir       669806 | poem         714052 |
|   15 | iron         616084 | iron         583088 | iron         539593 |
|   16 | trillium     519639 | trillium     458198 | trillium     537346 |
|   17 | rocket       435423 | rocket       437984 | rocket       464815 |
|   18 | astra        383302 | nickel       414460 | astra        379422 |
|   19 | nickel       382106 | astra        384163 | silent       363489 |
|   20 | silent       347182 | silent       357869 | nickel       229143 |
|   21 | oxidy        144163 | oxidy        128567 | oxidy        162522 |
|   22 | rouille       61639 | rouille       59604 | rouille       58889 |
|   23 | tide          12054 | tide          12327 | tide          11863 |
|   24 | summer-boot     12044 | summer-boot     12023 | summer-boot     11794 |
|   25 | may_minihttp         0 | may_minihttp         0 | may_minihttp         0 |
