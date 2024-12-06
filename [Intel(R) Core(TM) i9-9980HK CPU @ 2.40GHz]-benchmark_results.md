# Web Framework Benchmark Results

## Test Configuration

- Date: 2024-12-06 16:04:03
## Hardware Information
- CPU: Intel(R) Core(TM) i9-9980HK CPU @ 2.40GHz (16 cores)
- Memory: 7997 MB

# Performance Rankings (Requests/sec)

## Concurrency: 64

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    383815 | actix-web    395408 | xitca-web    423365 |
|    2 | xitca-web    367423 | xitca-web    381259 | actix-web    417104 |
|    3 | may_minihttp    355546 | may_minihttp    358337 | may_minihttp    366381 |
|    4 | ntex         333863 | ntex         338603 | ntex         352663 |
|    5 | iron         251300 | ohkami       277524 | ohkami       282801 |
|    6 | astra        238210 | iron         252380 | astra        269212 |
|    7 | ohkami       237962 | astra        246817 | iron         256125 |
|    8 | thruster     220208 | warp         225464 | thruster     236176 |
|    9 | argan        216517 | argan        217791 | axum         229781 |
|   10 | axum         204851 | axum         216717 | warp         223743 |
|   11 | warp         204120 | thruster     211552 | argan        220231 |
|   12 | saphir       195321 | nickel       199518 | saphir       210058 |
|   13 | poem         195310 | poem         193161 | poem         199161 |
|   14 | graphul      186929 | graphul      187761 | graphul      189744 |
|   15 | gotham       186782 | saphir       187462 | silent       188847 |
|   16 | silent       184963 | gotham       185853 | salvo        185810 |
|   17 | nickel       183370 | silent       184637 | gotham       185252 |
|   18 | viz          179213 | viz          179961 | viz          181858 |
|   19 | salvo        175748 | salvo        176289 | trillium     174631 |
|   20 | trillium     166095 | trillium     162135 | nickel       172142 |
|   21 | rocket       144042 | rocket       156073 | rocket       160181 |
|   22 | oxidy         73202 | oxidy         66534 | oxidy         85954 |
|   23 | rouille       26367 | rouille       26702 | rouille       25675 |
|   24 | summer-boot      1551 | tide           1546 | summer-boot      1547 |
|   25 | tide           1545 | summer-boot      1544 | tide           1545 |

## Concurrency: 256

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | actix-web    583554 | actix-web    602307 | actix-web    654809 |
|    2 | xitca-web    578002 | ntex         596615 | ntex         651529 |
|    3 | ntex         553458 | xitca-web    579048 | xitca-web    599253 |
|    4 | ohkami       486022 | ohkami       486438 | ohkami       577303 |
|    5 | may_minihttp    440869 | argan        443016 | may_minihttp    467908 |
|    6 | argan        430242 | may_minihttp    439415 | argan        458506 |
|    7 | thruster     416823 | thruster     423428 | thruster     457462 |
|    8 | warp         398586 | warp         421557 | axum         444664 |
|    9 | graphul      361284 | axum         379228 | warp         428395 |
|   10 | axum         360217 | graphul      351368 | graphul      385425 |
|   11 | viz          359971 | gotham       344658 | viz          383485 |
|   12 | poem         338094 | poem         339482 | saphir       367758 |
|   13 | saphir       334542 | viz          333406 | gotham       354944 |
|   14 | salvo        325214 | saphir       326703 | poem         354273 |
|   15 | gotham       320538 | salvo        324211 | salvo        301963 |
|   16 | trillium     261707 | trillium     266635 | trillium     281225 |
|   17 | iron         242400 | iron         253445 | iron         256565 |
|   18 | silent       234991 | silent       231187 | astra        245707 |
|   19 | rocket       219969 | rocket       227817 | rocket       244130 |
|   20 | astra        211625 | astra        219654 | silent       239066 |
|   21 | nickel       170503 | nickel       185319 | nickel       187790 |
|   22 | oxidy         75031 | oxidy         66980 | oxidy         83493 |
|   23 | rouille       25047 | rouille       25697 | rouille       25512 |
|   24 | tide           6026 | tide           6084 | tide           6079 |
|   25 | summer-boot      5989 | summer-boot      5974 | summer-boot      6009 |

## Concurrency: 512

| Rank | /fortunes | /json | /plaintext |
|------|-----------------|-----------------|-----------------|
|    1 | ohkami       622875 | ohkami       649054 | ohkami       748221 |
|    2 | actix-web    596296 | actix-web    630314 | actix-web    664460 |
|    3 | ntex         577376 | ntex         599575 | ntex         634989 |
|    4 | xitca-web    557853 | xitca-web    586139 | xitca-web    624983 |
|    5 | thruster     539953 | thruster     558398 | thruster     594186 |
|    6 | argan        496409 | argan        527776 | argan        575505 |
|    7 | may_minihttp    480957 | warp         519147 | warp         561852 |
|    8 | warp         474197 | may_minihttp    499691 | axum         544944 |
|    9 | axum         423357 | axum         448446 | may_minihttp    508954 |
|   10 | viz          422249 | viz          412277 | viz          457895 |
|   11 | graphul      406784 | poem         407887 | saphir       449997 |
|   12 | gotham       390258 | gotham       404994 | graphul      447735 |
|   13 | poem         389523 | graphul      401746 | gotham       435494 |
|   14 | salvo        384560 | salvo        392837 | poem         411629 |
|   15 | saphir       381008 | saphir       365776 | salvo        353488 |
|   16 | trillium     270461 | trillium     287616 | trillium     308753 |
|   17 | rocket       240993 | iron         247040 | iron         261924 |
|   18 | silent       231718 | rocket       235375 | rocket       256721 |
|   19 | iron         225562 | silent       231381 | astra        237155 |
|   20 | astra        208671 | astra        214502 | silent       234449 |
|   21 | nickel       123028 | nickel       190770 | nickel       135308 |
|   22 | oxidy         77006 | oxidy         67344 | oxidy         83231 |
|   23 | rouille       24938 | rouille       24550 | rouille       25429 |
|   24 | summer-boot     12012 | tide          12012 | summer-boot     11978 |
|   25 | tide          11910 | summer-boot     11837 | tide          11935 |
