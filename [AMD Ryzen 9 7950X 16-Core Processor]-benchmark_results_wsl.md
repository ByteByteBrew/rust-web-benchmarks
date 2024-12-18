# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-18 23:06:41
## Hardware Information
- CPU: AMD Ryzen 9 7950X 16-Core Processor (32 cores)
- Memory: 31699 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | trillium     502836 | trillium     505924 | trillium     515030 |
|    2 | xitca-web    423759 | xitca-web    459239 | xitca-web    427340 |
|    3 | ohkami       403202 | ntex         405670 | thruster     409103 |
|    4 | warp         402530 | thruster     404860 | warp         408770 |
|    5 | ntex         402318 | warp         404622 | ohkami       408246 |
|    6 | thruster     400744 | actix-web    402531 | ntex         404526 |
|    7 | axum         396915 | ohkami       400615 | axum         403956 |
|    8 | argan        395434 | argan        399269 | argan        403281 |
|    9 | poem         394659 | axum         397712 | actix-web    402894 |
|   10 | saphir       393120 | poem         395896 | poem         397224 |
|   11 | salvo        392363 | silent       393937 | saphir       397158 |
|   12 | silent       392177 | saphir       390075 | salvo        395276 |
|   13 | viz          391174 | viz          389009 | may_minihttp    394475 |
|   14 | gotham       385794 | gotham       387547 | silent       394377 |
|   15 | graphul      384388 | graphul      382973 | viz          390479 |
|   16 | may_minihttp    384124 | may_minihttp    382062 | gotham       388268 |
|   17 | rocket       383764 | rocket       381189 | graphul      385286 |
|   18 | actix-web    376917 | salvo        378121 | rocket       384807 |
|   19 | iron         324264 | iron         337266 | iron         338620 |
|   20 | nickel       247410 | nickel       258007 | nickel       249226 |
|   21 | oxidy        104286 | oxidy         84705 | oxidy        102513 |
|   22 | astra         83791 | astra         84161 | astra         83653 |
|   23 | rouille       28298 | rouille       28085 | rouille       28626 |
|   24 | tide           1216 | summer-boot      1219 | summer-boot      1220 |
|   25 | summer-boot      1216 | tide           1218 | tide           1217 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami       925799 | ohkami       906660 | ohkami       926993 |
|    2 | thruster     891897 | warp         905128 | thruster     914592 |
|    3 | warp         891548 | thruster     902961 | warp         913596 |
|    4 | argan        871909 | argan        890901 | argan        905661 |
|    5 | axum         858158 | axum         862544 | axum         882905 |
|    6 | viz          844961 | viz          855330 | viz          867328 |
|    7 | saphir       832809 | poem         840235 | saphir       861020 |
|    8 | poem         828931 | gotham       831613 | poem         858055 |
|    9 | gotham       827064 | graphul      830354 | gotham       844534 |
|   10 | graphul      826809 | saphir       789727 | salvo        837837 |
|   11 | salvo        807712 | salvo        788690 | graphul      837000 |
|   12 | xitca-web    718013 | silent       685028 | xitca-web    717998 |
|   13 | silent       683424 | trillium     660395 | silent       682007 |
|   14 | trillium     655525 | xitca-web    618940 | trillium     666882 |
|   15 | ntex         493765 | ntex         485844 | ntex         492008 |
|   16 | actix-web    462450 | actix-web    469544 | actix-web    481827 |
|   17 | may_minihttp    447269 | may_minihttp    449873 | may_minihttp    454074 |
|   18 | rocket       344984 | rocket       406677 | rocket       422757 |
|   19 | iron         328188 | iron         344892 | iron         345426 |
|   20 | nickel       195949 | nickel       191502 | nickel       191559 |
|   21 | oxidy        103599 | oxidy         85594 | oxidy         91929 |
|   22 | astra         83653 | astra         83654 | astra         84044 |
|   23 | rouille       27966 | rouille       28071 | rouille       28135 |
|   24 | tide           4711 | tide           4688 | tide           4661 |
|   25 | summer-boot      4656 | summer-boot      4680 | summer-boot      4653 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami      1311348 | ohkami      1271812 | ohkami      1306011 |
|    2 | thruster    1209618 | thruster    1264083 | thruster    1291039 |
|    3 | argan       1166566 | argan       1201999 | argan       1261337 |
|    4 | warp        1110836 | warp        1116223 | warp        1165178 |
|    5 | viz         1070235 | viz         1076345 | axum        1103909 |
|    6 | axum        1068837 | axum        1073117 | viz         1094170 |
|    7 | saphir      1054988 | gotham      1067856 | graphul     1079376 |
|    8 | poem        1049095 | salvo       1052851 | poem        1077829 |
|    9 | graphul     1046922 | saphir      1051297 | gotham      1073655 |
|   10 | salvo       1044849 | poem        1049345 | saphir      1069395 |
|   11 | gotham      1041725 | graphul     1045164 | salvo       1065337 |
|   12 | silent       720401 | silent       731034 | silent       724481 |
|   13 | trillium     705080 | trillium     713640 | trillium     714537 |
|   14 | xitca-web    602576 | xitca-web    600280 | xitca-web    583579 |
|   15 | ntex         569268 | ntex         574063 | ntex         580847 |
|   16 | actix-web    547509 | actix-web    535178 | actix-web    553325 |
|   17 | may_minihttp    512971 | may_minihttp    520943 | may_minihttp    532017 |
|   18 | rocket       371920 | iron         343922 | iron         342243 |
|   19 | iron         338148 | rocket       301796 | rocket       290328 |
|   20 | nickel       171889 | nickel       190774 | nickel       190855 |
|   21 | oxidy        105826 | oxidy         84809 | oxidy        102566 |
|   22 | astra         83829 | astra         84037 | astra         83836 |
|   23 | rouille       27656 | rouille       27588 | rouille       27546 |
|   24 | tide           9402 | tide           9450 | tide           9404 |
|   25 | summer-boot      9341 | summer-boot      9383 | summer-boot      9387 |
