-- some queries to explore the table

select * from actors limit 4;
select * from directors limit 5;
select * from movies m  limit 6;
select * from movies2actors ma limit 7;

-- average rating of all movies
create view average_rating as (select movieid, avg(CAST(rating as integer)) from u2base as u natural join movies group by movieid); 

select * from average_rating;

-- number of actors in each movie
create view number_of_actors as (select movieid, count(actorid) from actors natural join movies2actors group by movieid);
select * from number_of_actors;
-- number of ratings for each movieid 

create view number_of_ratings as (select movieid, count(CAST(rating as integer)) from u2base as u natural join movies group by movieid); 
select * from number_of_ratings;
-- number of ratings by each user 
select userid, count(movieid) from u2base ub natural join movies group by userid order by userid ASC;


-- 5
-- number of users who have rated atleast one movies 
select count(c.count)from (select count(u.userid) from users as u inner join u2base as ub on u.userid = ub.userid group by u.userid) as c;

-- 6
select * from movies where movieid not in (select movieid from u2base ub);

--7 
select * from movies2actors where movieid in (select movieid from average_rating order by avg desc limit 10);
