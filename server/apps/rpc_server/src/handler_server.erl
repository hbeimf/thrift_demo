-module(handler_server).
-include("msg_service_thrift.hrl").
-export([start/0, handle_function/2, say/1, stop/1, handle_error/2]).


debug(Info)->
    io:format("Debug info:~s~n",[Info]).

say(Name)->
    io:format("~n Line:~p~n", [?LINE]),
    Sentence = "Hello," ++ Name,
    debug(Sentence),
    BinSentence = list_to_binary(Sentence),
    BinSentence.

start()->
    start(9090).

start(Port)->
    Handler = ?MODULE,
    thrift_socket_server:start([{handler, Handler},
            {service, msg_service_thrift},
            {port, Port},
            {name, msg_server}]).

stop(Server)->
    thrift_socket_server:stop(Server).


handle_error(_P1, _P2) -> 
    % io:format("error : ~p ~n ", [{P1, P2}]),
    ok.

handle_function(hello, TheMessageRecord) ->
    %% unpack these or not, whatever.  Point is it's a record:
    % _Id = TheMessageRecord#message.id,
    % _Msg = TheMessageRecord#message.text,

    io:format("answer: ~p ~n ", [TheMessageRecord]),

    %% at this point you probably want to talk to a pool of gen_servers
    %% or something like that.

    %% send a reply per the service definition in thrift/example.thrift:
    {reply, #'Message'{id = 1, text = <<"Thanks!">>}};

handle_function(_Function, _Args) ->
    {reply, #'Message'{id = 404, text = <<"not found!">>}}.