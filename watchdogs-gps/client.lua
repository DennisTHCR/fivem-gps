local path = {}
local blip_location = nil
local recompute_threshold = 50.0 -- Distance threshold to trigger a path recomputation
local node_reach_distance = 10.0 -- Distance to consider a node as passed

-- Event listener to receive the path from the server
RegisterNetEvent('pathfinding:updatePath')
AddEventHandler('pathfinding:updatePath', function(newPath)
    path = newPath
end)

Citizen.CreateThread(function()
    while true do
        local playerPos = GetEntityCoords(PlayerPedId())
        local blip = GetFirstBlipInfoId(8)

        if blip ~= 0 then
            local new_blip_location = GetBlipCoords(blip)

            -- If the blip location has changed, recompute the path
            if new_blip_location ~= blip_location then
                blip_location = new_blip_location
                TriggerServerEvent('pathfinding:updateLocation', PlayerId(), playerPos, blip_location)
            end

            -- If there’s a path, manage it dynamically
            if #path > 0 then
                local nextNode = path[1]
                local nextNodePos = vector3(nodes[nextNode].position.x, nodes[nextNode].position.y, nodes[nextNode].position.z)

                -- Remove the node if the player has reached it
                if #(playerPos - nextNodePos) < node_reach_distance then
                    table.remove(path, 1)
                end

                -- Recompute the path if the player is too far from the current path
                local closestNodeDist = #(playerPos - nextNodePos)
                if closestNodeDist > recompute_threshold then
                    TriggerServerEvent('pathfinding:updateLocation', PlayerId(), playerPos, blip_location)
                end
            end
        else
            path = {} -- Clear path if no blip is present
        end

        Citizen.Wait(500) -- Check every half second for efficiency
    end
end)

Citizen.CreateThread(function()
    while true do
        local playerPos = GetEntityCoords(PlayerPedId())

        -- Draw markers for the remaining path
        if path ~= nil then
            for _, node in ipairs(path) do
                local nodePos = vector3(nodes[node].position.x, nodes[node].position.y, nodes[node].position.z)
                if #(playerPos - nodePos) < 100.0 then -- Adjust the drawing distance threshold
                    DrawMarker(2, nodePos.x, nodePos.y, nodePos.z, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 200, 0, 210, 255, false, true, 2, false, nil, nil, false)
                end
            end
        end
        Citizen.Wait(0)
    end
end)
