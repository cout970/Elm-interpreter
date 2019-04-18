{
  "header": {
    "name": "Maybe",
    "exposing": {
      "Just": [
        {
          "Adt": [
            "Maybe",
            "All"
          ]
        },
        {
          "Definition": "andThen"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "map2"
        },
        {
          "Definition": "map3"
        },
        {
          "Definition": "map4"
        },
        {
          "Definition": "map5"
        },
        {
          "Definition": "withDefault"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Basics"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Adt": [
              "Bool",
              "All"
            ]
          }
        ]
      }
    }
  ],
  "statements": [
    {
      "Adt": [
        "Maybe",
        [
          "a"
        ],
        [
          [
            "Just",
            [
              {
                "Var": "a"
              }
            ]
          ],
          [
            "Nothing",
            []
          ]
        ]
      ]
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Var": "a"
                }
              ]
            }
          ]
        },
        "name": "withDefault",
        "patterns": [
          {
            "Var": "default"
          },
          {
            "Var": "maybe"
          }
        ],
        "expr": {
          "Case": [
            [
              1562,
              1626
            ],
            {
              "Ref": [
                [
                  1567,
                  1572
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "value"
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      1596,
                      1601
                    ],
                    "value"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      1619,
                      1626
                    ],
                    "default"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "map",
        "patterns": [
          {
            "Var": "f"
          },
          {
            "Var": "maybe"
          }
        ],
        "expr": {
          "Case": [
            [
              1895,
              1977
            ],
            {
              "Ref": [
                [
                  1900,
                  1905
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "value"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      1933,
                      1947
                    ],
                    {
                      "Ref": [
                        [
                          1933,
                          1938
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Application": [
                        [
                          1939,
                          1946
                        ],
                        {
                          "Ref": [
                            [
                              1939,
                              1940
                            ],
                            "f"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1941,
                              1946
                            ],
                            "value"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      1970,
                      1977
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "value"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "value"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map2",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "ma"
          },
          {
            "Var": "mb"
          }
        ],
        "expr": {
          "Case": [
            [
              2445,
              2598
            ],
            {
              "Ref": [
                [
                  2450,
                  2452
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2477,
                      2484
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2506,
                      2598
                    ],
                    {
                      "Ref": [
                        [
                          2511,
                          2513
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              2546,
                              2553
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "Just",
                            [
                              {
                                "Var": "b"
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              2583,
                              2598
                            ],
                            {
                              "Ref": [
                                [
                                  2583,
                                  2588
                                ],
                                "Just"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  2589,
                                  2597
                                ],
                                {
                                  "Application": [
                                    [
                                      2589,
                                      2597
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2589,
                                          2593
                                        ],
                                        "func"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2594,
                                          2595
                                        ],
                                        "a"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2596,
                                      2597
                                    ],
                                    "b"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Var": "value"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "value"
                              }
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map3",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "ma"
          },
          {
            "Var": "mb"
          },
          {
            "Var": "mc"
          }
        ],
        "expr": {
          "Case": [
            [
              2708,
              2956
            ],
            {
              "Ref": [
                [
                  2713,
                  2715
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2740,
                      2747
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2769,
                      2956
                    ],
                    {
                      "Ref": [
                        [
                          2774,
                          2776
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              2809,
                              2816
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "Just",
                            [
                              {
                                "Var": "b"
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              2846,
                              2956
                            ],
                            {
                              "Ref": [
                                [
                                  2851,
                                  2853
                                ],
                                "mc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    "Nothing",
                                    []
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2894,
                                      2901
                                    ],
                                    "Nothing"
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    "Just",
                                    [
                                      {
                                        "Var": "c"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2939,
                                      2956
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2939,
                                          2944
                                        ],
                                        "Just"
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          2945,
                                          2955
                                        ],
                                        {
                                          "Application": [
                                            [
                                              2945,
                                              2955
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  2945,
                                                  2955
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      2945,
                                                      2949
                                                    ],
                                                    "func"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      2950,
                                                      2951
                                                    ],
                                                    "a"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  2952,
                                                  2953
                                                ],
                                                "b"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              2954,
                                              2955
                                            ],
                                            "c"
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Var": "value"
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Maybe",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Tag": [
                                "Maybe",
                                [
                                  {
                                    "Var": "value"
                                  }
                                ]
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map4",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "ma"
          },
          {
            "Var": "mb"
          },
          {
            "Var": "mc"
          },
          {
            "Var": "md"
          }
        ],
        "expr": {
          "Case": [
            [
              3085,
              3444
            ],
            {
              "Ref": [
                [
                  3090,
                  3092
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      3117,
                      3124
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      3146,
                      3444
                    ],
                    {
                      "Ref": [
                        [
                          3151,
                          3153
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3186,
                              3193
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "Just",
                            [
                              {
                                "Var": "b"
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              3223,
                              3444
                            ],
                            {
                              "Ref": [
                                [
                                  3228,
                                  3230
                                ],
                                "mc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    "Nothing",
                                    []
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3271,
                                      3278
                                    ],
                                    "Nothing"
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    "Just",
                                    [
                                      {
                                        "Var": "c"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3316,
                                      3444
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3321,
                                          3323
                                        ],
                                        "md"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            "Nothing",
                                            []
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3372,
                                              3379
                                            ],
                                            "Nothing"
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            "Just",
                                            [
                                              {
                                                "Var": "d"
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              3425,
                                              3444
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3425,
                                                  3430
                                                ],
                                                "Just"
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  3431,
                                                  3443
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      3431,
                                                      3443
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          3431,
                                                          3443
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              3431,
                                                              3443
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  3431,
                                                                  3435
                                                                ],
                                                                "func"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  3436,
                                                                  3437
                                                                ],
                                                                "a"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              3438,
                                                              3439
                                                            ],
                                                            "b"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          3440,
                                                          3441
                                                        ],
                                                        "c"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      3442,
                                                      3443
                                                    ],
                                                    "d"
                                                  ]
                                                }
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "e"
                                },
                                {
                                  "Var": "value"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Maybe",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Fun": [
                                {
                                  "Tag": [
                                    "Maybe",
                                    [
                                      {
                                        "Var": "e"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Tag": [
                                    "Maybe",
                                    [
                                      {
                                        "Var": "value"
                                      }
                                    ]
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map5",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Var": "ma"
          },
          {
            "Var": "mb"
          },
          {
            "Var": "mc"
          },
          {
            "Var": "md"
          },
          {
            "Var": "me"
          }
        ],
        "expr": {
          "Case": [
            [
              3592,
              4078
            ],
            {
              "Ref": [
                [
                  3597,
                  3599
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      3624,
                      3631
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      3653,
                      4078
                    ],
                    {
                      "Ref": [
                        [
                          3658,
                          3660
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3693,
                              3700
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            "Just",
                            [
                              {
                                "Var": "b"
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              3730,
                              4078
                            ],
                            {
                              "Ref": [
                                [
                                  3735,
                                  3737
                                ],
                                "mc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    "Nothing",
                                    []
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3778,
                                      3785
                                    ],
                                    "Nothing"
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    "Just",
                                    [
                                      {
                                        "Var": "c"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3823,
                                      4078
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3828,
                                          3830
                                        ],
                                        "md"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            "Nothing",
                                            []
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3879,
                                              3886
                                            ],
                                            "Nothing"
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            "Just",
                                            [
                                              {
                                                "Var": "d"
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Case": [
                                            [
                                              3932,
                                              4078
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3937,
                                                  3939
                                                ],
                                                "me"
                                              ]
                                            },
                                            [
                                              [
                                                {
                                                  "Adt": [
                                                    "Nothing",
                                                    []
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      3996,
                                                      4003
                                                    ],
                                                    "Nothing"
                                                  ]
                                                }
                                              ],
                                              [
                                                {
                                                  "Adt": [
                                                    "Just",
                                                    [
                                                      {
                                                        "Var": "e"
                                                      }
                                                    ]
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      4057,
                                                      4078
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4057,
                                                          4062
                                                        ],
                                                        "Just"
                                                      ]
                                                    },
                                                    {
                                                      "Application": [
                                                        [
                                                          4063,
                                                          4077
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              4063,
                                                              4077
                                                            ],
                                                            {
                                                              "Application": [
                                                                [
                                                                  4063,
                                                                  4077
                                                                ],
                                                                {
                                                                  "Application": [
                                                                    [
                                                                      4063,
                                                                      4077
                                                                    ],
                                                                    {
                                                                      "Application": [
                                                                        [
                                                                          4063,
                                                                          4077
                                                                        ],
                                                                        {
                                                                          "Ref": [
                                                                            [
                                                                              4063,
                                                                              4067
                                                                            ],
                                                                            "func"
                                                                          ]
                                                                        },
                                                                        {
                                                                          "Ref": [
                                                                            [
                                                                              4068,
                                                                              4069
                                                                            ],
                                                                            "a"
                                                                          ]
                                                                        }
                                                                      ]
                                                                    },
                                                                    {
                                                                      "Ref": [
                                                                        [
                                                                          4070,
                                                                          4071
                                                                        ],
                                                                        "b"
                                                                      ]
                                                                    }
                                                                  ]
                                                                },
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      4072,
                                                                      4073
                                                                    ],
                                                                    "c"
                                                                  ]
                                                                }
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4074,
                                                                  4075
                                                                ],
                                                                "d"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              4076,
                                                              4077
                                                            ],
                                                            "e"
                                                          ]
                                                        }
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ]
                                            ]
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "andThen",
        "patterns": [
          {
            "Var": "callback"
          },
          {
            "Var": "maybeValue"
          }
        ],
        "expr": {
          "Case": [
            [
              5175,
              5282
            ],
            {
              "Ref": [
                [
                  5180,
                  5190
                ],
                "maybeValue"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "value"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5228,
                      5242
                    ],
                    {
                      "Ref": [
                        [
                          5228,
                          5236
                        ],
                        "callback"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5237,
                          5242
                        ],
                        "value"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5275,
                      5282
                    ],
                    "Nothing"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isJust",
        "patterns": [
          {
            "Var": "maybe"
          }
        ],
        "expr": {
          "Case": [
            [
              5406,
              5472
            ],
            {
              "Ref": [
                [
                  5411,
                  5416
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      "Wildcard"
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      5440,
                      5444
                    ],
                    "True"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5467,
                      5472
                    ],
                    "False"
                  ]
                }
              ]
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "b"
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "destruct",
        "patterns": [
          {
            "Var": "default"
          },
          {
            "Var": "func"
          },
          {
            "Var": "maybe"
          }
        ],
        "expr": {
          "Case": [
            [
              5548,
              5618
            ],
            {
              "Ref": [
                [
                  5553,
                  5558
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    "Just",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5582,
                      5588
                    ],
                    {
                      "Ref": [
                        [
                          5582,
                          5586
                        ],
                        "func"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5587,
                          5588
                        ],
                        "a"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5611,
                      5618
                    ],
                    "default"
                  ]
                }
              ]
            ]
          ]
        }
      }
    }
  ]
}