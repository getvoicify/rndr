import { Routes } from '@angular/router';
import { isAwsCredentialValidGuard, hasDepsGuard, hasExtDepsGuard } from './has-deps.guard';

export const appRoutes: Routes = [

  {
    path: 'missing-deps',
    loadComponent: () => import('./missing-deps/missing-deps.component').then((m) => m.MissingDepsComponent),
  },
  {
    path: 'missing-vars',
    loadComponent: () => import('./missing-vars/missing-vars.component').then((m) => m.MissingVarsComponent),
  },
  {
    path: 'stacks',
    loadChildren: () => import('./stacks/stacks-routing.module').then((m) => m.StacksRoutingModule),
  },
  {
    path: '',
    loadComponent: () => import('./home/home.component').then((m) => m.HomeComponent),
    canActivate: [hasDepsGuard, isAwsCredentialValidGuard, hasExtDepsGuard],
    children: [
      {
        path: 'jobs',
        loadComponent: () => import('./job-list/job-list.component').then((m) => m.JobListComponent),
      },
      {
        path: 'settings',
        loadChildren: () => import('./settings/settings-routing.module').then((m) => m.SettingsRoutingModule),
      },
      {
        path: '',
        loadChildren: () => import('./render/component.routes').then((m) => m.RenderRoutingModule),
      },
    ]
  },
  {
    path: '**',
    redirectTo: '',
  },
];
