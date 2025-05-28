-module(some).
-export([some_fn/0]).


some_fn()->
    Pid = spawn(fun() ->
            receive
                Msg -> io:format("Erlang got: ~p~n", [Msg])
            end
    end),

    Pid ! "Hello from Erlang".