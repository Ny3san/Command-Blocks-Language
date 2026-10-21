scoreboard players set #t2 cb302f60846c41 1
scoreboard players operation #t3 cb302f60846c41 = #v0 cb302f60846c41
scoreboard players operation #t3 cb302f60846c41 += #t2 cb302f60846c41
scoreboard players operation #v0 cb302f60846c41 = #t3 cb302f60846c41
tellraw @a [{"text":""},{"text":"Pontuação: "},{"score":{"name":"#v0","objective":"cb302f60846c41"}}]
