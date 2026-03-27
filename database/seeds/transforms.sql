-- ==========================================================================
-- SEED DEFAULT TRANSFORMS
-- ==========================================================================
-- Safe to run multiple times.

DO $$
DECLARE
    t_id BIGINT;
BEGIN

    -- Gain
    IF NOT EXISTS (SELECT 1 FROM transforms WHERE name = 'Gain') THEN
        INSERT INTO transforms (name, description)
        VALUES ('Gain', 'Amplifies or attenuates the signal level')
        RETURNING transform_id INTO t_id;

        INSERT INTO transform_ports (transform_id, name, direction, port_order) VALUES
            (t_id, 'In',  'input',  0),
            (t_id, 'Out', 'output', 0);
    END IF;

    -- Low-Pass Filter
    IF NOT EXISTS (SELECT 1 FROM transforms WHERE name = 'Low-Pass Filter') THEN
        INSERT INTO transforms (name, description)
        VALUES ('Low-Pass Filter', 'Attenuates frequencies above the cutoff frequency')
        RETURNING transform_id INTO t_id;

        INSERT INTO transform_ports (transform_id, name, direction, port_order) VALUES
            (t_id, 'In',  'input',  0),
            (t_id, 'Out', 'output', 0);
    END IF;

    -- High-Pass Filter
    IF NOT EXISTS (SELECT 1 FROM transforms WHERE name = 'High-Pass Filter') THEN
        INSERT INTO transforms (name, description)
        VALUES ('High-Pass Filter', 'Attenuates frequencies below the cutoff frequency')
        RETURNING transform_id INTO t_id;

        INSERT INTO transform_ports (transform_id, name, direction, port_order) VALUES
            (t_id, 'In',  'input',  0),
            (t_id, 'Out', 'output', 0);
    END IF;

    -- Compressor
    IF NOT EXISTS (SELECT 1 FROM transforms WHERE name = 'Compressor') THEN
        INSERT INTO transforms (name, description)
        VALUES ('Compressor', 'Reduces dynamic range by attenuating loud signals')
        RETURNING transform_id INTO t_id;

        INSERT INTO transform_ports (transform_id, name, direction, port_order) VALUES
            (t_id, 'In',  'input',  0),
            (t_id, 'Out', 'output', 0);
    END IF;

    -- Reverb
    IF NOT EXISTS (SELECT 1 FROM transforms WHERE name = 'Reverb') THEN
        INSERT INTO transforms (name, description)
        VALUES ('Reverb', 'Simulates acoustic reflections in a space')
        RETURNING transform_id INTO t_id;

        INSERT INTO transform_ports (transform_id, name, direction, port_order) VALUES
            (t_id, 'In',  'input',  0),
            (t_id, 'Out', 'output', 0);
    END IF;

    -- Mixer
    IF NOT EXISTS (SELECT 1 FROM transforms WHERE name = 'Mixer') THEN
        INSERT INTO transforms (name, description)
        VALUES ('Mixer', 'Blends two audio signals into one output')
        RETURNING transform_id INTO t_id;

        INSERT INTO transform_ports (transform_id, name, direction, port_order) VALUES
            (t_id, 'In A', 'input',  0),
            (t_id, 'In B', 'input',  1),
            (t_id, 'Out',  'output', 0);
    END IF;

END $$;
