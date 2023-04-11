import { Route, RouterModule, Routes } from '@angular/router';
import { NgModule } from '@angular/core';
import { hasStacksRepoGuard } from '../has-deps.guard';

const resetRoute: Route = {
  path: '**',
  redirectTo: '',
}
export const routes: Routes = [
  {
    path: '',
    loadComponent: () => import('./render-shell/render-shell.component').then(m => m.RenderShellComponent),
    canActivate: [hasStacksRepoGuard],
    children: [
      {
        path: '',
        loadComponent: () => import('./start-render/start-render.component').then(m => m.StartRenderComponent),
      },
      resetRoute,
    ]
  },
  resetRoute
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
})
export class RenderRoutingModule {}