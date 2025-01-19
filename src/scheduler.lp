% Parameters
day(1..dmax).      % dmax days in the period

% partners of a date are occupied for that day
occupied(I, J, D) :- date(I, J, D).
occupied(J, I, D) :- date(I, J, D).

% A date can happen on a day if two individuals are in a relationship
{ date(I, J, D) } :- day(D), relation(I, J, _).

% Constraints
% Ensure each individual dates at most one person per day
{ date(I, J, D): relation(I, _, _)} :- relation(_, J, _), day(D).
{ date(I, J, D): relation(_, J, _)} :- relation(I, _, _), day(D).

% Ensure at least one date per relationship
:- relation(I, J, _), not 1 { date(I, J, D) : day(D) }.

% only one date per person per day
:- date(I, J, D), occupied(I, J', D), J != J'.
:- date(J, I, D), occupied(I, J', D), J != J'.

% Compute weighted waiting times
% Waiting time between consecutive dates
waiting_time(I, J, WT) :- date(I, J, D1), date(I, J, D2), D1 < D2, not {date(I, J, D) : D1 < D, D < D2}, relation(I, J, W), WT = (D2 - D1) * W.

% Cyclic waiting time between the last date and the first date of the next period
last_date(I, J, D) :- date(I, J, D), D >= D': date(I, J, D').
first_date(I, J, D) :- date(I, J, D), D <= D': date(I, J, D').
waiting_time(I, J, WT) :- last_date(I, J, D1), first_date(I, J, D2), relation(I, J, W), WT = (dmax - D1 + D2 + 1) * W.

% find maximal weighted waiting time
max_waiting_time(WT) :- waiting_time(_, _, WT), WT >= WT' : waiting_time(_, _, WT').

% Minimize maximum weighted waiting time
#minimize { WT : max_waiting_time(WT) }.

% Show the dates
#show.
#show date/3.