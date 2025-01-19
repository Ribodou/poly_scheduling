% Parameters
day(1..dmax).      % dmax days in the period

% partners of a meeting are occupied for that day
occupied(I, J, D) :- meet(I, J, D).
occupied(J, I, D) :- meet(I, J, D).

% A meeting can happen on a day if two individuals are in a relationship
{ meet(I, J, D) } :- day(D), relation(I, J, _).

% Constraints
% Ensure each individual meets at most one person per day
{ meet(I, J, D): relation(I, _, _)} :- relation(_, J, _), day(D).
{ meet(I, J, D): relation(_, J, _)} :- relation(I, _, _), day(D).

% Ensure at least one meeting per relationship
:- relation(I, J, _), not 1 { meet(I, J, D) : day(D) }.

% only one meeting per person per day
:- meet(I, J, D), occupied(I, J', D), J != J'.
:- meet(J, I, D), occupied(I, J', D), J != J'.

% Compute weighted waiting times
% Waiting time between consecutive meetings
waiting_time(I, J, WT) :- meet(I, J, D1), meet(I, J, D2), D1 < D2, relation(I, J, W), WT = (D2 - D1) * W.

% Cyclic waiting time between the last meeting and the first meeting of the next period
last_meeting(I, J, D) :- meet(I, J, D), D >= D': meet(I, J, D').
first_meeting(I, J, D) :- meet(I, J, D), D <= D': meet(I, J, D').
waiting_time(I, J, WT) :- last_meeting(I, J, D1), first_meeting(I, J, D2), relation(I, J, W), WT = (dmax - D1 + D2 + 1) * W.

% find maximal weighted waiting time
max_waiting_time(WT) :- waiting_time(_, _, WT), WT >= WT' : waiting_time(_, _, WT').

% Minimize maximum weighted waiting time
#minimize { WT : max_waiting_time(WT) }.

% Show the meetings
#show.
#show meet/3.
% #show occupied/3.