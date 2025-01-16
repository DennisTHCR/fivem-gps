path = {}
blip_location = nil

-- Event listener to receive the path from the server
RegisterNetEvent('pathfinding:updatePath')
AddEventHandler('pathfinding:updatePath', function(newPath)
    path = newPath
end)

Citizen.CreateThread(function()
    while true do
        local blip = GetFirstBlipInfoId(8)
        if blip ~= 0 then
            local new_blip_location = GetBlipCoords(blip)
            if new_blip_location ~= blip_location then
                blip_location = new_blip_location
            end

            local player_location = GetEntityCoords(PlayerPedId())

            -- Send updated location and blip location to the server
            TriggerServerEvent('pathfinding:updateLocation', PlayerId(), player_location, blip_location)
        else
            path = {}
        end

        Citizen.Wait(1000) -- Adjust as needed for update frequency
    end
end)

Citizen.CreateThread(function()
    while true do
        local playerPos = GetEntityCoords(PlayerPedId())
        for _, node in ipairs(path) do
            local distance = #(playerPos - vector3(nodes[node].position.x, nodes[node].position.y, nodes[node].position.z))
            if distance < 100.0 then -- Adjust the distance threshold as needed
                DrawMarker(2, nodes[node].position.x, nodes[node].position.y, nodes[node].position.z, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 200, 0, 210, 255, false, true, 2, false, nil, nil, false)
            end
        end
        Citizen.Wait(0)
    end
end)
