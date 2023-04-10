import { createPropertySelectors, createSelector, Selector } from '@ngxs/store';
import { AppState } from './app.state';
import { AppStateModel, isGlobalLoading, isLocalLoading, Loading } from './app-state.model';


export class AppStateSelectors {
  private static slice = createPropertySelectors<AppStateModel>(AppState)
  @Selector([AppStateSelectors.slice.loading])
  static isLoadingGlobal(loading: Loading[]) {
    return loading.filter(
      l => isGlobalLoading(l)
    ).length > 0;
  }

  static isLoading(key: string) {
    return createSelector([AppStateSelectors.slice.loading], (loading: Loading[]) => loading.filter(
      l => isLocalLoading(l) && l.id === key
    ).length > 0);
  }

}