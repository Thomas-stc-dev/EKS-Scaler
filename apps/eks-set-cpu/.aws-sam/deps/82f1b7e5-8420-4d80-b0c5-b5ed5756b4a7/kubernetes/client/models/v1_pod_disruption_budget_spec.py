# coding: utf-8

"""
    Kubernetes

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)  # noqa: E501

    The version of the OpenAPI document: release-1.31
    Generated by: https://openapi-generator.tech
"""


import pprint
import re  # noqa: F401

import six

from kubernetes.client.configuration import Configuration


class V1PodDisruptionBudgetSpec(object):
    """NOTE: This class is auto generated by OpenAPI Generator.
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    """
    Attributes:
      openapi_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    openapi_types = {
        'max_unavailable': 'object',
        'min_available': 'object',
        'selector': 'V1LabelSelector',
        'unhealthy_pod_eviction_policy': 'str'
    }

    attribute_map = {
        'max_unavailable': 'maxUnavailable',
        'min_available': 'minAvailable',
        'selector': 'selector',
        'unhealthy_pod_eviction_policy': 'unhealthyPodEvictionPolicy'
    }

    def __init__(self, max_unavailable=None, min_available=None, selector=None, unhealthy_pod_eviction_policy=None, local_vars_configuration=None):  # noqa: E501
        """V1PodDisruptionBudgetSpec - a model defined in OpenAPI"""  # noqa: E501
        if local_vars_configuration is None:
            local_vars_configuration = Configuration()
        self.local_vars_configuration = local_vars_configuration

        self._max_unavailable = None
        self._min_available = None
        self._selector = None
        self._unhealthy_pod_eviction_policy = None
        self.discriminator = None

        if max_unavailable is not None:
            self.max_unavailable = max_unavailable
        if min_available is not None:
            self.min_available = min_available
        if selector is not None:
            self.selector = selector
        if unhealthy_pod_eviction_policy is not None:
            self.unhealthy_pod_eviction_policy = unhealthy_pod_eviction_policy

    @property
    def max_unavailable(self):
        """Gets the max_unavailable of this V1PodDisruptionBudgetSpec.  # noqa: E501

        An eviction is allowed if at most \"maxUnavailable\" pods selected by \"selector\" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with \"minAvailable\".  # noqa: E501

        :return: The max_unavailable of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :rtype: object
        """
        return self._max_unavailable

    @max_unavailable.setter
    def max_unavailable(self, max_unavailable):
        """Sets the max_unavailable of this V1PodDisruptionBudgetSpec.

        An eviction is allowed if at most \"maxUnavailable\" pods selected by \"selector\" are unavailable after the eviction, i.e. even in absence of the evicted pod. For example, one can prevent all voluntary evictions by specifying 0. This is a mutually exclusive setting with \"minAvailable\".  # noqa: E501

        :param max_unavailable: The max_unavailable of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :type: object
        """

        self._max_unavailable = max_unavailable

    @property
    def min_available(self):
        """Gets the min_available of this V1PodDisruptionBudgetSpec.  # noqa: E501

        An eviction is allowed if at least \"minAvailable\" pods selected by \"selector\" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying \"100%\".  # noqa: E501

        :return: The min_available of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :rtype: object
        """
        return self._min_available

    @min_available.setter
    def min_available(self, min_available):
        """Sets the min_available of this V1PodDisruptionBudgetSpec.

        An eviction is allowed if at least \"minAvailable\" pods selected by \"selector\" will still be available after the eviction, i.e. even in the absence of the evicted pod.  So for example you can prevent all voluntary evictions by specifying \"100%\".  # noqa: E501

        :param min_available: The min_available of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :type: object
        """

        self._min_available = min_available

    @property
    def selector(self):
        """Gets the selector of this V1PodDisruptionBudgetSpec.  # noqa: E501


        :return: The selector of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :rtype: V1LabelSelector
        """
        return self._selector

    @selector.setter
    def selector(self, selector):
        """Sets the selector of this V1PodDisruptionBudgetSpec.


        :param selector: The selector of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :type: V1LabelSelector
        """

        self._selector = selector

    @property
    def unhealthy_pod_eviction_policy(self):
        """Gets the unhealthy_pod_eviction_policy of this V1PodDisruptionBudgetSpec.  # noqa: E501

        UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction. Current implementation considers healthy pods, as pods that have status.conditions item with type=\"Ready\",status=\"True\".  Valid policies are IfHealthyBudget and AlwaysAllow. If no policy is specified, the default behavior will be used, which corresponds to the IfHealthyBudget policy.  IfHealthyBudget policy means that running pods (status.phase=\"Running\"), but not yet healthy can be evicted only if the guarded application is not disrupted (status.currentHealthy is at least equal to status.desiredHealthy). Healthy pods will be subject to the PDB for eviction.  AlwaysAllow policy means that all running pods (status.phase=\"Running\"), but not yet healthy are considered disrupted and can be evicted regardless of whether the criteria in a PDB is met. This means perspective running pods of a disrupted application might not get a chance to become healthy. Healthy pods will be subject to the PDB for eviction.  Additional policies may be added in the future. Clients making eviction decisions should disallow eviction of unhealthy pods if they encounter an unrecognized policy in this field.  This field is beta-level. The eviction API uses this field when the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).  # noqa: E501

        :return: The unhealthy_pod_eviction_policy of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :rtype: str
        """
        return self._unhealthy_pod_eviction_policy

    @unhealthy_pod_eviction_policy.setter
    def unhealthy_pod_eviction_policy(self, unhealthy_pod_eviction_policy):
        """Sets the unhealthy_pod_eviction_policy of this V1PodDisruptionBudgetSpec.

        UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods should be considered for eviction. Current implementation considers healthy pods, as pods that have status.conditions item with type=\"Ready\",status=\"True\".  Valid policies are IfHealthyBudget and AlwaysAllow. If no policy is specified, the default behavior will be used, which corresponds to the IfHealthyBudget policy.  IfHealthyBudget policy means that running pods (status.phase=\"Running\"), but not yet healthy can be evicted only if the guarded application is not disrupted (status.currentHealthy is at least equal to status.desiredHealthy). Healthy pods will be subject to the PDB for eviction.  AlwaysAllow policy means that all running pods (status.phase=\"Running\"), but not yet healthy are considered disrupted and can be evicted regardless of whether the criteria in a PDB is met. This means perspective running pods of a disrupted application might not get a chance to become healthy. Healthy pods will be subject to the PDB for eviction.  Additional policies may be added in the future. Clients making eviction decisions should disallow eviction of unhealthy pods if they encounter an unrecognized policy in this field.  This field is beta-level. The eviction API uses this field when the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).  # noqa: E501

        :param unhealthy_pod_eviction_policy: The unhealthy_pod_eviction_policy of this V1PodDisruptionBudgetSpec.  # noqa: E501
        :type: str
        """

        self._unhealthy_pod_eviction_policy = unhealthy_pod_eviction_policy

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.openapi_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, V1PodDisruptionBudgetSpec):
            return False

        return self.to_dict() == other.to_dict()

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        if not isinstance(other, V1PodDisruptionBudgetSpec):
            return True

        return self.to_dict() != other.to_dict()
