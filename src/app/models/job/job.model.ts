export interface JobRaw {
  additional_file_count: number;
  cloudid:               string;
  description:           string;
  endframe:              number;
  error:                 string;
  id:                    string;
  is_uploaded:           string;
  key:                   string;
  package:               string;
  parent:                string;
  percentage:            number;
  samples:               number;
  scene:                 string;
  source_blend_path:     string;
  startedAt:             string;
  startframe:            number;
  status:                string;
  step:                  number;
  stoppedAt:             string;
  use_large_disk:        string;
  xres:                  number;
  yres:                  number;
}

export interface Job {

  additional_file_count: number;
  cloudid:               string;
  description:           string;
  endframe:              number;
  error:                 string;
  id:                    string;
  is_uploaded:           boolean;
  key:                   string;
  package:               string;
  parent:                string;
  percentage:            number;
  samples:               number;
  scene:                 string;
  source_blend_path:     string;
  startedAt?:             Date;
  startframe:            number;
  status:                string;
  step:                  number;
  stoppedAt?:             Date;
  use_large_disk:        boolean;
  xres:                  number;
  yres:                  number;
}

export const jobMapper = (job: JobRaw): Job => ({
  ...job,
  is_uploaded: job.is_uploaded === 'True' || job.is_uploaded === 'true',
  use_large_disk: job.use_large_disk === 'True' || job.use_large_disk === 'true',
  startedAt: job.startedAt ? new Date(parseInt(job.startedAt, 10)) : undefined,
  stoppedAt: job.stoppedAt ?  new Date(parseInt(job.stoppedAt, 10)) : undefined,
});