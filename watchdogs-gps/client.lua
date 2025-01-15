function get_next(guid)
    local neighbors = {}
    local current_node = nodes[guid]
    if links[guid] then
        for _, link in ipairs(links[guid]) do
            for _, ref in ipairs(link.refs) do
                local neighbor_node = nodes[ref]
                if neighbor_node then
                    local dx = neighbor_node.position.x - current_node.position.x
                    local dy = neighbor_node.position.y - current_node.position.y
                    local dz = neighbor_node.position.z - current_node.position.z
                    local distance = math.sqrt(dx * dx + dy * dy + dz * dz)
                    table.insert(neighbors, { guid = ref, cost = distance })
                end
            end
        end
    end
    return neighbors
end

-- A-star algorithm
function astar(start, goal)
    local open_set = { [start] = true }
    local came_from = {}
    local g_score = { [start] = 0 }
    local f_score = { [start] = heuristic_cost(start, goal) }
    
    while next(open_set) do
        local current = nil
        for node in pairs(open_set) do
            if not current or f_score[node] < f_score[current] then
                current = node
            end
        end
        
        if current == goal then
            local path = {}
            local cost = g_score[current]
            while current do
                table.insert(path, 1, current)
                current = came_from[current]
            end
            return path, cost
        end
        
        open_set[current] = nil
        
        for _, neighbor in ipairs(get_next(current)) do
            local tentative_g_score = g_score[current] + neighbor.cost
            if not g_score[neighbor.guid] or tentative_g_score < g_score[neighbor.guid] then
                came_from[neighbor.guid] = current
                g_score[neighbor.guid] = tentative_g_score
                f_score[neighbor.guid] = g_score[neighbor.guid] + heuristic_cost(neighbor.guid, goal)
                open_set[neighbor.guid] = true
            end
        end
    end
    return nil, nil
end

-- Heuristic function (Euclidean distance)
function heuristic_cost(guid, goal)
    local node = nodes[guid]
    local goal_node = nodes[goal]
    local dx = node.position.x - goal_node.position.x
    local dy = node.position.y - goal_node.position.y
    local dz = node.position.z - goal_node.position.z
    return math.sqrt(dx * dx + dy * dy + dz * dz)
end

function get_closest_node(position)
    local closest_node = nil
    local closest_distance = math.huge

    for guid, node in pairs(nodes) do
        if node and node.position then
            local dx = node.position.x - position.x
            local dy = node.position.y - position.y
            local dz = node.position.z - position.z
            local distance = math.sqrt(dx * dx + dy * dy + dz * dz)

            if distance < closest_distance then
                closest_distance = distance
                closest_node = guid
            end
        end
    end

    return closest_node
end

path = {}

Citizen.CreateThread(function()
    while true do
        blip = GetFirstBlipInfoId(8)
        if blip ~= 0 then
            blip_location = GetBlipCoords(blip)
            player_location = GetEntityCoords(PlayerPedId())
            guid_close = get_closest_node(player_location)
            guid_far = get_closest_node(blip_location)
            path, cost = astar(guid_close, guid_far)
        else
            path = {}
        end
        Citizen.Wait(1000)
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