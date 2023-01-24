export interface CreateStackResultModel {
  stackName: string;
  stackStatus: string | { [key: string]: string };
}

export interface ErrorResultModel extends CreateStackResultModel {
  stackStatus: { Error: string };
}

export const isErrorResult = (result: CreateStackResultModel): result is ErrorResultModel => {
  return typeof result.stackStatus !== 'string' && 'Error' in result.stackStatus;
}

export const isPendingResult = (result: CreateStackResultModel): result is CreateStackResultModel => {
  return typeof result.stackStatus === 'string' && result.stackStatus === 'Pending';
}

export const isCompleteResult = (result: CreateStackResultModel): result is CreateStackResultModel => {
  return result.stackStatus === 'Done';
}

export const isProcessingResult = (result: CreateStackResultModel): result is CreateStackResultModel => {
  return result.stackStatus === 'Processing';
}

export const isInfoResult = (result: CreateStackResultModel): result is CreateStackResultModel => {
  return typeof result.stackStatus === 'string' && result.stackStatus === 'Info';
}