tellraw @a [{"text":""},{"text":"Sistema iniciado"}]
function command_blocks:internal/t5
function command_blocks:internal/t5
function command_blocks:internal/t5
function command_blocks:internal/t5
function command_blocks:internal/t5
scoreboard players set #t6 cb302f60846c41 5
scoreboard players set #t7 cb302f60846c41 0
execute if score #v0 cb302f60846c41 >= #t6 cb302f60846c41 run scoreboard players set #t7 cb302f60846c41 1
scoreboard players operation #t8 cb302f60846c41 = #t7 cb302f60846c41
execute if score #t8 cb302f60846c41 matches 1 run function command_blocks:internal/t9
