%%
%% Autogenerated by Thrift Compiler (0.13.0)
%%
%% DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
%%

-ifndef(_msg_types_included).
-define(_msg_types_included, yeah).

%% struct 'msg.Message'

-record('msg.Message', {'id' :: integer() | 'undefined',
                        'text' :: string() | binary() | 'undefined'}).
-type 'msg.Message'() :: #'msg.Message'{}.

%% struct 'msg.UserInfo'

-record('msg.UserInfo', {'uid' :: integer() | 'undefined',
                         'name' :: string() | binary() | 'undefined'}).
-type 'msg.UserInfo'() :: #'msg.UserInfo'{}.

%% struct 'msg.ServerReply'

-record('msg.ServerReply', {'code' :: integer() | 'undefined',
                            'text' :: string() | binary() | 'undefined'}).
-type 'msg.ServerReply'() :: #'msg.ServerReply'{}.

-endif.
