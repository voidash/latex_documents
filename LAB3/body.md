## Update the tables to add primary and foreign key constraints 

```sql
ALTER table actors add primary key (id);

ALTER table directors add primary key (id);

ALTER table movies add primary key (id);

alter table roles add primary key (id);

alter table movies_directors 
add foreign key (movie_id) REFERENCES movies(id);

alter table movies_directors 
add foreign key (director_id) references directors (id);

ALTER table roles 
add foreign key (movie_id) references movies (id) ,
add foreign key (actor_id) references actors (id);

ALTER table directors_genres 
add foreign key directors_genres (director_id) references directors (id);

ALTER table movies_genres 
add foreign key movies_genres (movie_id) references movies (id) ;
```


## Remove Film_count from actors table

```sql
alter table actors 
drop column film_count;
```

## Write SQL query for the following

### Find the number of actors, movies, genres and directors
```sql
/* 1 */
select (select count(*) from movies) as movie_count,
  (select count(*) from actors) as ac,
  (select count(DISTINCT genre) from directors_genres dg ) as dgc,
  (select count(DISTINCT genre) from movies_genres mg ) as mgc,
  (select count(*) from directors) as dc;
``` 

|mc|ac|dc|mgc|dc|
|-----------|-----------|--------------------|-----------------|--------------|
|36         |1907       |19                  |16               |34            |


### Find the full name, and role of all the actors who played in Titanic.

```sql
 /* 2 */
 select
	m.name,
	a.first_name,
	a.last_name,
	r.role
from
	roles as r
inner join actors as a on
	a.id = r.actor_id
inner join movies as m on
	m.id = r.movie_id
where
	m.name like 'Titanic'; 

```

|name|first_name|last_name|role|
|----|----------|---------|----|
|Titanic|Lewis     |Abernathy|Lewis Bodine|
|Titanic|Seth (I)  |Adkins   |Slovakian three-year-old boy|
|Titanic|Scott G.  |Anderson |Lookout Fredrick Fleet|
|Titanic|Kris      |Andersson|Steerage dancer|
|Titanic|Richard (II)|Ashton   |Carpenter John Hutchinson|


### Find the number of movies in all genres.

```sql
/* 3 */
select mg.genre, count(m.name) as movies_on_that_genre 
from movies as m inner join movies_genres as mg 
on mg.movie_id=m.id
group by mg.genre;

```

|genre|movies_on_that_genre|
|-----|--------------------|
|Action|8                   |
|Adventure|5                   |
|Animation|2                   |
|Comedy|11                  |
|Crime|12                  |
|Drama|17                  |
|Family|3                   |
|Fantasy|5                   |
|Horror|3                   |
|Music|1                   |
|Musical|1                   |
|Mystery|6                   |
|Romance|5                   |
|Sci-Fi|6                   |
|Thriller|17                  |
|War  |1                   |


### Find the average number of movies played by actors 

```sql
/* 4 */
select avg(t.number_of_movies_played) from (select
	r.actor_id,
	count(r.movie_id) as number_of_movies_played
from
	roles as r
group by
	r.actor_id) as t;
```

|avg(t.number of movies played)|
|------------------------------|
| 1.043|

### Find the average number of actors in a movie

```sql
/* 5 */
select avg(t.number_of_actors) from (
select r.movie_id,count(r.actor_id) as number_of_actors from roles as r
group by r.movie_id) as t;
```

|avg(t.number_of_actors)|
|------------------------------|
| 55.25|

### Find top 5 movies based on the rank.

```sql
/* 6 */
select * from movies order by rank DESC limit 5;
```

|id |name|year|rank|
|---|----|----|----|
|297838|Shawshank Redemption, The|1994|9.0 |
|130128|Godfather, The|1972|9.0 |
|313459|Star Wars|1977|8.8 |
|267038|Pulp Fiction|1994|8.7 |
|210511|Memento|2000|8.7 |

### Find 20 directors who have directed at least 2 movies

```sql
/* 7 */
select d.id,concat(d.first_name, " ", d.last_name) as name, count(md.movie_id) as number_of_movies_directed from  movies_directors md 
inner join directors d on d.id=md.director_id 
group by md.director_id 
order by number_of_movies_directed DESC limit 20;
```
|id |name|number_of_movies_directed|
|---|----|-------------------------|
|78273|Quentin Tarantino|4                        |
|15093|Joel Coen|2                        |
|58201|Christopher Nolan|2                        |
|15092|Ethan Coen|2                        |
|11652|James (I) Cameron|2                        |
|2931|Darren Aronofsky|1                        |
|38746|Mike (I) Judge|1                        |
|88802|Unknown Director|1                        |
|15906|Sofia Coppola|1                        |
|65940|Rob Reiner|1                        |
|35838|John (I) Hughes|1                        |
|83616|Andy Wachowski|1                        |
|56332|John Musker|1                        |
|28395|Mel (I) Gibson|1                        |
|14927|Ron Clements|1                        |
|46315|Jay Levey|1                        |
|22104|Clint Eastwood|1                        |
|74758|Steven Soderbergh|1                        |
|9247|Zach Braff|1                        |
|41975|David Koepp|1                        |


### Find all movies played by Kevin Bacon.

```sql
/* 8 */
select a.first_name,a.last_name,m.name from roles as r 
inner join actors as a on r.actor_id = a.id 
inner join movies m on m.id=r.movie_id 
where a.first_name LIKE 'Kevin' AND a.last_name  Like 'Bacon';
```

|first_name|last_name|name|
|----------|---------|----|
|Kevin     |Bacon    |Animal House|
|Kevin     |Bacon    |Apollo 13|
|Kevin     |Bacon    |Few Good Men, A|
|Kevin     |Bacon    |Footloose|
|Kevin     |Bacon    |Hollow Man|
|Kevin     |Bacon    |JFK |
|Kevin     |Bacon    |Mystic River|
|Kevin     |Bacon    |Planes, Trains & Automobiles|
|Kevin     |Bacon    |Stir of Echoes|

### Find all movies released from 1990 to 2000.

```sql
/* 9 */
select * from movies m where m.year >= 1990 and m.year < 2000;
```
|id |name|year|rank|
|---|----|----|----|
|18979|Apollo 13|1995|7.5 |
|46169|Braveheart|1995|8.3 |
|109093|Fargo|1996|8.2 |
|111813|Few Good Men, A|1992|7.5 |
|112290|Fight Club|1999|8.5 |
|167324|JFK |1991|7.8 |
|207992|Matrix, The|1999|8.5 |
|238695|Office Space|1999|7.6 |
|254943|Pi  |1998|7.5 |
|267038|Pulp Fiction|1994|8.7 |
|276217|Reservoir Dogs|1992|8.3 |
|297838|Shawshank Redemption, The|1994|9.0 |
|314965|Stir of Echoes|1999|7.0 |
|333856|Titanic|1997|6.9 |
