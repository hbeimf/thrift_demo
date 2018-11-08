-ifndef(_msg_types_included).
-define(_msg_types_included, yeah).

%% struct 'msg.Message'

-record('msg.Message', {'id' :: integer(),
                        'text' :: string() | binary()}).
-type 'msg.Message'() :: #'msg.Message'{}.

%% struct 'msg.UserInfo'

-record('msg.UserInfo', {'uid' :: integer(),
                         'name' :: string() | binary()}).
-type 'msg.UserInfo'() :: #'msg.UserInfo'{}.

%% struct 'msg.ServerReply'

-record('msg.ServerReply', {'code' :: integer(),
                            'text' :: string() | binary()}).
-type 'msg.ServerReply'() :: #'msg.ServerReply'{}.

-endif.
