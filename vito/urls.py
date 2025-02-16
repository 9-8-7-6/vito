from django.contrib import admin
from django.conf import settings
from django.conf.urls.static import static
from django.urls import path, include
from rest_framework import permissions
from drf_spectacular.views import SpectacularAPIView, SpectacularRedocView, SpectacularSwaggerView
from bookkeeping.urls import router

urlpatterns = [
   path("admin/", admin.site.urls),
   path('api/schema/', SpectacularAPIView.as_view(), name='schema'),
   path('api/schema/swagger-ui/', SpectacularSwaggerView.as_view(url_name='schema'), name='swagger-ui'),
   path('api/schema/redoc/', SpectacularRedocView.as_view(url_name='schema'), name='redoc'),
   path("", include(router.urls)),
]

if settings.DEBUG:
    urlpatterns += static(settings.STATIC_URL, document_root=settings.STATIC_ROOT)
