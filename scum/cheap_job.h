#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>

typedef struct Job {
  int task_id;
  int priority;
  int tie_break;
} Job;

typedef struct CheapJob cheap_job_t;

extern cheap_job_t* new_cheap_job(void);

extern void free_cheap_job(cheap_job_t *);

extern void push_cheap_job(cheap_job_t *, Job *);

extern Job* pop_cheap_job(cheap_job_t *);
