-ifndef(_msg_types_included).
-define(_msg_types_included, yeah).

%% struct 'msg.Message'

-record('msg.Message', {'id' :: integer(),
                        'text' :: string() | binary()}).
-type 'msg.Message'() :: #'msg.Message'{}.

-endif.
