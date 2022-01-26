#include <stdio.h>
#include <stdlib.h>
#include "rusty.h"
#include "cheap_job.h"

Job* make_job(int pid, int pri, int tie) {
  Job* job = malloc(sizeof(Job));
  job->task_id = pid;
  job->priority = pri;
  job->tie_break = tie;

  return job;
}

int main(int argc, char* argv[]) {
  printf("Hello, this is C\n");
  greet_generic();

  Job* j1 = make_job(1, 10, 1);
  Job* j2 = make_job(2, 42, 1);
  Job* j3 = make_job(3, 5, 1);
  Job* j4 = make_job(4, 3, 1);
  Job* j5 = make_job(5, 4, 1);
  Job* j6 = make_job(6, 2, 1);
  Job* j7 = make_job(7, 8, 1);
  Job* j8 = make_job(8, 12, 1);

  cheap_job_t* queue = new_cheap_job();

  push_cheap_job(queue, j1);
  push_cheap_job(queue, j2);
  push_cheap_job(queue, j3);
  push_cheap_job(queue, j4);
  push_cheap_job(queue, j5);

  printf("Should be j4: %d\n", pop_cheap_job(queue)->task_id);
  printf("Should be j5: %d\n", pop_cheap_job(queue)->task_id);

  push_cheap_job(queue, j6);
  push_cheap_job(queue, j7);

  printf("Should be j6: %d\n", pop_cheap_job(queue)->task_id);

  push_cheap_job(queue, j8);

  printf("Should be j3: %d\n", pop_cheap_job(queue)->task_id);
  printf("Should be j7: %d\n", pop_cheap_job(queue)->task_id);
  printf("Should be j1: %d\n", pop_cheap_job(queue)->task_id);
  printf("Should be j8: %d\n", pop_cheap_job(queue)->task_id);
  printf("Should be j2: %d\n", pop_cheap_job(queue)->task_id);

  printf("Should be null: %d\n", pop_cheap_job(queue) == NULL ? 1 : 0);
  printf("Should be null: %d\n", pop_cheap_job(queue) == NULL ? 1 : 0);

  free_cheap_job(queue);
}
