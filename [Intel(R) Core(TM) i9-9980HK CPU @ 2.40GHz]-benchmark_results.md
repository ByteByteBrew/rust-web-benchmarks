# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-17 15:43:36
## Hardware Information
- CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 cores)
- Memory: 16049 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    419737 | xitca-web    453430 | actix-web    475226 |
|    2 | may_minihttp    379562 | actix-web    427182 | xitca-web    470331 |
|    3 | xitca-web    358998 | may_minihttp    386178 | may_minihttp    413270 |
|    4 | ntex         357266 | ntex         365347 | ntex         377336 |
|    5 | ohkami       283669 | ohkami       288016 | ohkami       305323 |
|    6 | iron         256612 | iron         262460 | iron         264809 |
|    7 | astra        231887 | thruster     240048 | thruster     262698 |
|    8 | warp         225575 | astra        236049 | astra        247412 |
|    9 | thruster     225321 | warp         233201 | warp         245729 |
|   10 | argan        221982 | argan        224420 | argan        229492 |
|   11 | axum         208173 | poem         208677 | viz          224681 |
|   12 | poem         206780 | viz          207717 | axum         224228 |
|   13 | viz          205111 | axum         204456 | poem         209172 |
|   14 | saphir       199756 | graphul      200022 | saphir       209167 |
|   15 | graphul      197517 | nickel       198756 | graphul      207754 |
|   16 | salvo        197285 | salvo        195768 | salvo        205398 |
|   17 | gotham       192815 | gotham       193262 | gotham       201217 |
|   18 | nickel       188293 | saphir       191440 | silent       186089 |
|   19 | silent       178417 | silent       181972 | nickel       182316 |
|   20 | trillium     161680 | trillium     167336 | rocket       171052 |
|   21 | rocket       157820 | rocket       153852 | trillium     167868 |
|   22 | oxidy         82344 | oxidy         70944 | oxidy         93195 |
|   23 | rouille       26351 | rouille       27279 | rouille       27374 |
|   24 | summer-boot      1557 | summer-boot      1553 | tide           1556 |
|   25 | tide           1555 | tide           1553 | summer-boot      1555 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    648856 | actix-web    670438 | actix-web    738742 |
|    2 | xitca-web    629627 | ntex         646286 | ntex         700127 |
|    3 | ntex         615677 | xitca-web    627416 | xitca-web    674750 |
|    4 | ohkami       559517 | ohkami       582854 | ohkami       619621 |
|    5 | may_minihttp    483181 | may_minihttp    484578 | may_minihttp    511362 |
|    6 | thruster     469944 | thruster     483034 | thruster     488648 |
|    7 | argan        460053 | argan        459949 | argan        487386 |
|    8 | warp         439497 | warp         448664 | warp         469105 |
|    9 | graphul      400820 | viz          395859 | axum         456575 |
|   10 | viz          397681 | axum         389366 | viz          429505 |
|   11 | axum         389965 | graphul      385796 | graphul      412228 |
|   12 | gotham       367687 | gotham       374607 | saphir       403303 |
|   13 | poem         363983 | poem         372631 | gotham       393422 |
|   14 | saphir       359218 | salvo        356461 | poem         393398 |
|   15 | salvo        354970 | saphir       344129 | salvo        374581 |
|   16 | trillium     292101 | trillium     299254 | trillium     314628 |
|   17 | iron         244914 | iron         269547 | iron         264322 |
|   18 | rocket       239984 | rocket       240649 | rocket       253065 |
|   19 | silent       218285 | silent       216709 | astra        233362 |
|   20 | astra        207347 | astra        204037 | silent       225886 |
|   21 | nickel       174132 | nickel       178835 | nickel       205567 |
|   22 | oxidy         81258 | oxidy         72465 | oxidy         90092 |
|   23 | rouille       26621 | rouille       26857 | rouille       27325 |
|   24 | summer-boot      6102 | tide           6155 | summer-boot      6218 |
|   25 | tide           6013 | summer-boot      6001 | tide           6143 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami       726673 | ohkami       761513 | ohkami       808637 |
|    2 | actix-web    659695 | actix-web    688966 | actix-web    756463 |
|    3 | ntex         626111 | xitca-web    644733 | ntex         714988 |
|    4 | xitca-web    622789 | ntex         640281 | xitca-web    675908 |
|    5 | thruster     593347 | thruster     623161 | thruster     653038 |
|    6 | argan        561608 | warp         578952 | argan        625256 |
|    7 | warp         540951 | argan        573421 | warp         619555 |
|    8 | may_minihttp    536198 | may_minihttp    539171 | may_minihttp    580734 |
|    9 | axum         481247 | axum         480642 | axum         561091 |
|   10 | graphul      463698 | viz          459065 | viz          520724 |
|   11 | viz          461448 | graphul      445980 | graphul      487477 |
|   12 | saphir       446745 | gotham       445601 | saphir       477359 |
|   13 | poem         444287 | poem         440742 | gotham       461063 |
|   14 | gotham       427605 | salvo        428703 | poem         460895 |
|   15 | salvo        425554 | saphir       392347 | salvo        453965 |
|   16 | trillium     306231 | trillium     303735 | trillium     327648 |
|   17 | iron         253088 | iron         258616 | iron         269686 |
|   18 | rocket       249094 | rocket       251162 | rocket       265467 |
|   19 | astra        206098 | silent       219839 | astra        232557 |
|   20 | silent       202986 | astra        204124 | silent       211001 |
|   21 | nickel       126084 | nickel       156960 | nickel       147583 |
|   22 | oxidy         78661 | oxidy         71604 | oxidy         88546 |
|   23 | rouille       25919 | rouille       26500 | rouille       26518 |
|   24 | summer-boot     12104 | tide          12197 | tide          12024 |
|   25 | tide          11737 | summer-boot     11955 | summer-boot     11924 |
