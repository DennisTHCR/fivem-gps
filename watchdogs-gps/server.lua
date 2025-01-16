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

                    -- Modify distance based on the Speed attribute
                    local speed = neighbor_node.attributes and tonumber(neighbor_node.attributes["Speed"])
                    if speed then
                        if speed == 2 then
                            distance = distance / 2
                        elseif speed == 3 then
                            distance = distance / 3
                        end
                        -- No need to modify if Speed is not 2 or 3
                    end

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

RegisterNetEvent('pathfinding:updateLocation')
AddEventHandler('pathfinding:updateLocation', function(playerId, playerPosition, blipLocation)
    local src = source

    -- Get the closest nodes to the player's position and the blip location
    local startNode = get_closest_node(playerPosition)
    local goalNode = get_closest_node(blipLocation)

    if startNode and goalNode then
        -- Run A* algorithm
        local path, cost = astar(startNode, goalNode)

        -- Send the path back to the client
        TriggerClientEvent('pathfinding:updatePath', src, path)
    else
        -- Send an empty path if no valid nodes are found
        TriggerClientEvent('pathfinding:updatePath', src, {})
    end
end)

